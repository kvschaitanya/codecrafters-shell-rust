use is_executable::is_executable;
use std::env::{current_dir, set_current_dir, split_paths, var};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

struct ShellLexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Iterator for ShellLexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: String = String::new();
        let mut is_quoted = false;

        while let Some(c) = self.chars.next() {
            if c == '\'' {
                is_quoted = !is_quoted;
                continue;
            }
            if !is_quoted {
                if c == ' ' && token.is_empty() {
                    continue;
                } else if c == ' ' {
                    break;
                }
            }
            token.push(c);
        }
        if token.is_empty() { None } else { Some(token) }
    }
}

fn external_command_path(command: &str) -> Option<std::path::PathBuf> {
    let paths = var("PATH").unwrap_or_default();

    split_paths(&paths).find_map(|path| {
        let file_path = path.join(command);
        (file_path.is_file() && is_executable(&file_path)).then_some(file_path)
    })
}

fn main() {
    let mut input: String = String::new();
    let builtin_commands = ["exit", "echo", "type", "pwd", "cd"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();.

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the input");

        let command: Vec<String> = ShellLexer {
            chars: input.trim().chars(),
        }
        .collect();

        let bridge: Vec<&str> = command.iter().map(|s| s.as_str()).collect();
        match bridge.as_slice() {
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

            ["pwd", ..] => println!("{}", current_dir().unwrap_or_default().display()),

            ["cd", args @ ..] => {
                let home_dir: String;

                let expanded_path: String;

                let target = match args.first() {
                    None => {
                        home_dir = var("HOME").unwrap_or_default();
                        home_dir.as_str()
                    }
                    Some(&path) if path.starts_with("~") => {
                        home_dir = var("HOME").unwrap_or_default();
                        expanded_path = path.replacen("~", home_dir.as_str(), 1);
                        expanded_path.as_str()
                    }
                    Some(&path) => path,
                };

                if set_current_dir(target).is_err() {
                    eprintln!("cd: {target}: No such file or directory");
                };
            }

            [cmd, args @ ..] => match external_command_path(cmd) {
                Some(_) => {
                    if let Err(e) = Command::new(cmd).args(args).status() {
                        eprintln!("{e}");
                    }
                }
                None => println!("{cmd}: not found"),
            },
        }
    }
}
