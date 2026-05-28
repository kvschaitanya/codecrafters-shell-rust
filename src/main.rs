use is_executable::is_executable;
use std::env::{current_dir, set_current_dir, split_paths, var};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

struct ShellLexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> ShellLexer<'a> {
    fn double_quotes(&mut self, token: &mut String) {}
}

impl<'a> Iterator for ShellLexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: String = String::new();

        while let Some(c) = self.chars.next() {
            match c {
                '\\' => {
                    if let Some(ch) = self.chars.next() {
                        token.push(ch);
                    }
                }
                '\'' => token.extend(self.chars.by_ref().take_while(|&ch| ch != '\'')),
                '"' => token.extend(self.chars.by_ref().take_while(|&ch| ch != '"')),
                //'"' => self.double_quotes(&mut token),
                ' ' if token.is_empty() => continue,
                ' ' => break,
                _ => token.push(c),
            }
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

trait LexerExt {
    fn tokenize(&self) -> ShellLexer<'_>;
}

impl LexerExt for str {
    fn tokenize(&self) -> ShellLexer<'_> {
        ShellLexer {
            chars: self.chars(),
        }
    }
}

fn main() {
    let mut input: String = String::new();
    let builtin_commands = ["exit", "echo", "type", "pwd", "cd"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the input");

        let mut tokens = input.trim().tokenize();

        match tokens.next().unwrap_or_default().as_str() {
            "" => continue,
            "exit" => break,

            "echo" => {
                println!("{}", tokens.collect::<Vec<String>>().join(" "));
            }

            "type" => match tokens.next() {
                Some(cmd) if builtin_commands.contains(&cmd.as_str()) => {
                    println!("{} is a shell builtin", cmd);
                }
                Some(cmd) => match external_command_path(&cmd) {
                    Some(p) => println!("{cmd} is {}", p.display()),
                    None => println!("{}: not found", cmd),
                },
                None => println!(": not found"),
            },

            "pwd" => println!("{}", current_dir().unwrap_or_default().display()),

            "cd" => {
                let target = match tokens.next() {
                    None => var("HOME").unwrap_or_default(),
                    Some(path) if path.starts_with("~") => {
                        path.replacen("~", var("HOME").unwrap_or_default().as_str(), 1)
                    }
                    Some(path) => path.to_string(),
                };

                if set_current_dir(&target).is_err() {
                    eprintln!("cd: {target}: No such file or directory");
                };
            }

            cmd => match external_command_path(cmd) {
                Some(_) => {
                    if let Err(e) = Command::new(cmd).args(tokens).status() {
                        eprintln!("{e}");
                    }
                }
                None => println!("{cmd}: not found"),
            },
        }
    }
}
