use is_executable::is_executable;
use std::env::{split_paths, var};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let builtin_commands = ["exit", "echo", "type"];

        let mut input: String = String::new();
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
                    let paths = var("PATH").unwrap_or_default();

                    let file_path = split_paths(&paths).find_map(|path| {
                        let file = path.join(cmd);
                        if file.is_file() && is_executable(&file) {
                            Some(file)
                        } else {
                            None
                        }
                    });

                    match file_path {
                        Some(p) => println!("{cmd} is {}", p.display()),
                        None => println!("{}: not found", cmd),
                    }
                }
            }
            [unknown_cmd, ..] => println!("{unknown_cmd}: not found"),
        }
    }
}
