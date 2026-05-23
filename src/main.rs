use is_executable::is_executable;
use std::env::{split_paths, var};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

fn external_command_path(command: &str) -> Option<std::path::PathBuf> {
    let paths = var("PATH").unwrap_or_default();

    split_paths(&paths).find_map(|path| {
        let file_path = path.join(command);
        (file_path.is_file() && is_executable(&file_path)).then_some(file_path)
    })
}

fn main() {
    let mut input: String = String::new();
    let builtin_commands = ["exit", "echo", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the input");

        let command: Vec<&str> = input.split_whitespace().collect();

        match command.as_slice() {
            [] => continue,
            ["exit", ..] => break,
            ["echo", args @ ..] => {
                println!("{}", args.join(" "));
            }
            ["type", cmd, ..] => {
                if builtin_commands.contains(cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    match external_command_path(cmd) {
                        Some(p) => println!("{cmd} is {}", p.display()),
                        None => println!("{}: not found", cmd),
                    }
                }
            }
            [cmd, args @ ..] => match external_command_path(cmd) {
                Some(exe_path) => {
                    if let Err(e) = Command::new(exe_path).args(args).status() {
                        eprintln!("{e}");
                    }
                }
                None => println!("{cmd}: not found"),
            },
        }
    }
}
