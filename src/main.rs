use is_executable::is_executable;
use std::fs;
use std::io::stderr;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{
    env::{current_dir, set_current_dir, split_paths, var},
    process::exit,
};

struct ShellLexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> ShellLexer<'a> {
    fn double_quotes(&mut self, token: &mut String) {
        while let Some(c) = self.chars.next() {
            match c {
                '\\' => match self.chars.next() {
                    Some(ch @ ('"' | '\\')) => token.push(ch),
                    Some(other) => {
                        token.push('\\');
                        token.push(other)
                    }
                    None => token.push('\\'),
                },
                '"' => break,
                _ => token.push(c),
            }
        }
    }
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
                //'"' => token.extend(self.chars.by_ref().take_while(|&ch| ch != '"')),
                '"' => self.double_quotes(&mut token),
                ' ' if token.is_empty() => continue,
                ' ' => break,
                _ => token.push(c),
            }
        }
        if token.is_empty() { None } else { Some(token) }
    }
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

enum OutputTarget {
    Stdout,
    File(String),
}

enum ErrorTarget {
    Stderr,
    File(String),
}

struct ShellCommand {
    command: String,
    args: Vec<String>,
    output: OutputTarget,
    error: ErrorTarget,
}

fn parse_commands(mut tokens: impl Iterator<Item = String>) -> Vec<ShellCommand> {
    let mut commands = vec![];

    while let Some(command) = tokens.next() {
        let mut args: Vec<String> = vec![];
        let mut output = OutputTarget::Stdout;
        let mut error = ErrorTarget::Stderr;

        while let Some(token) = tokens.next() {
            match token.as_str() {
                ">" | "1>" => {
                    if let Some(file) = tokens.next() {
                        output = OutputTarget::File(file);
                    }
                }
                "2>" => {
                    if let Some(file) = tokens.next() {
                        error = ErrorTarget::File(file)
                    }
                }
                _ => args.push(token),
            };
        }

        commands.push(ShellCommand {
            command,
            args,
            output,
            error,
        });
    }
    commands
}

fn external_command_path(command: &str) -> Option<std::path::PathBuf> {
    let paths = var("PATH").unwrap_or_default();

    split_paths(&paths).find_map(|path| {
        let file_path = path.join(command);
        (file_path.is_file() && is_executable(&file_path)).then_some(file_path)
    })
}

fn execute_command(command: ShellCommand) {
    let builtin_commands = ["exit", "echo", "type", "pwd", "cd"];

    let mut output: Box<dyn Write> = match &command.output {
        OutputTarget::Stdout => Box::new(io::stdout()),
        OutputTarget::File(file) => Box::new(fs::File::create(file).unwrap()),
    };

    let mut error: Box<dyn Write> = match &command.error {
        ErrorTarget::Stderr => Box::new(io::stderr()),
        ErrorTarget::File(file) => Box::new(fs::File::create(file).unwrap()),
    };

    match command.command.as_str() {
        "exit" => exit(0),
        "echo" => {
            if let Err(e) = writeln!(output, "{}", command.args.join(" ")) {
                writeln!(error, "{}", e);
            }
        }

        "type" => {
            for arg in command.args {
                if builtin_commands.contains(&arg.as_str()) {
                    if let Err(e) = writeln!(output, "{} is a shell builtin", arg) {
                        writeln!(error, "{}", e);
                    }
                } else {
                    match external_command_path(&arg) {
                        Some(p) => {
                            if let Err(e) = writeln!(output, "{arg} is {}", p.display()) {
                                writeln!(error, "{}", e);
                            }
                        }
                        None => {
                            writeln!(error, "{}: not found", arg);
                        }
                    }
                }
            }
        }

        "pwd" => {
            if let Err(e) = writeln!(output, "{}", current_dir().unwrap_or_default().display()) {
                writeln!(error, "{e}");
            }
        }

        "cd" => {
            let target = match command.args.first() {
                None => var("HOME").unwrap_or_default(),
                Some(path) if path.starts_with("~") => {
                    path.replacen("~", var("HOME").unwrap_or_default().as_str(), 1)
                }
                Some(path) => path.to_string(),
            };

            if set_current_dir(&target).is_err() {
                writeln!(error, "cd: {target}: No such file or directory");
            };
        }

        cmd => match external_command_path(cmd) {
            Some(_) => {
                let mut process = Command::new(cmd);
                process.args(command.args);
                match command.output {
                    OutputTarget::Stdout => process.stdout(io::stdout()),
                    OutputTarget::File(file) => process.stdout(fs::File::create(file).unwrap()),
                };
                match command.error {
                    ErrorTarget::Stderr => process.stderr(io::stderr()),
                    ErrorTarget::File(file) => process.stderr(fs::File::create(file).unwrap()),
                };
                if let Err(e) = process.status() {
                    writeln!(error, "{e}");
                }
            }
            None => {
                writeln!(error, "{cmd}: not found");
            }
        },
    }
}

fn main() {
    let mut input: String = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the input");

        for command in parse_commands(input.trim().tokenize()) {
            execute_command(command);
        }
    }
}
