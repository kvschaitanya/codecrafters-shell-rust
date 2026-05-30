use crate::parser::*;
use is_executable::is_executable;
use std::fs;
use std::io::{self, Write, stderr};
use std::process::Command;
use std::{
    env::{current_dir, set_current_dir, split_paths, var},
    process::exit,
};

fn external_command_path(command: &str) -> Option<std::path::PathBuf> {
    let paths = var("PATH").unwrap_or_default();

    split_paths(&paths).find_map(|path| {
        let file_path = path.join(command);
        (file_path.is_file() && is_executable(&file_path)).then_some(file_path)
    })
}

pub fn execute_command(command: ShellCommand) {
    let builtin_commands = ["exit", "echo", "type", "pwd", "cd"];

    let mut output_stream: Box<dyn Write> = match &command.output {
        OutputTarget::Stdout => Box::new(io::stdout()),
        OutputTarget::File(file) => Box::new(fs::File::create(file).unwrap()),
        OutputTarget::AppendFile(file) => {
            Box::new(fs::OpenOptions::new().append(true).open(file).unwrap())
        }
    };

    let mut error_stream: Box<dyn Write> = match &command.error {
        ErrorTarget::Stderr => Box::new(io::stderr()),
        ErrorTarget::File(file) => Box::new(fs::File::create(file).unwrap()),
        ErrorTarget::AppendFile(file) => {
            Box::new(fs::OpenOptions::new().append(true).open(file).unwrap())
        }
    };

    match command.command.as_str() {
        "exit" => exit(0),
        "echo" => {
            if let Err(e) = writeln!(output_stream, "{}", command.args.join(" ")) {
                let _ = writeln!(error_stream, "{}", e);
            }
        }

        "type" => {
            for arg in command.args {
                if builtin_commands.contains(&arg.as_str()) {
                    if let Err(e) = writeln!(output_stream, "{} is a shell builtin", arg) {
                        let _ = writeln!(error_stream, "{}", e);
                    }
                } else {
                    match external_command_path(&arg) {
                        Some(p) => {
                            if let Err(e) = writeln!(output_stream, "{arg} is {}", p.display()) {
                                let _ = writeln!(error_stream, "{}", e);
                            }
                        }
                        None => {
                            let _ = writeln!(error_stream, "{}: not found", arg);
                        }
                    }
                }
            }
        }

        "pwd" => {
            if let Err(e) = writeln!(
                output_stream,
                "{}",
                current_dir().unwrap_or_default().display()
            ) {
                let _ = writeln!(error_stream, "{e}");
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
                let _ = writeln!(error_stream, "cd: {target}: No such file or directory");
            };
        }

        cmd => match external_command_path(cmd) {
            Some(_) => {
                let mut process = Command::new(cmd);
                process.args(command.args);
                match command.output {
                    OutputTarget::Stdout => process.stdout(io::stdout()),
                    OutputTarget::File(file) => process.stdout(fs::File::create(file).unwrap()),
                    OutputTarget::AppendFile(file) => process.stdout(
                        fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(file)
                            .unwrap(),
                    ),
                };
                match command.error {
                    ErrorTarget::Stderr => process.stderr(io::stderr()),
                    ErrorTarget::File(file) => process.stderr(fs::File::create(file).unwrap()),
                    ErrorTarget::AppendFile(file) => process.stderr(
                        fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(file)
                            .unwrap(),
                    ),
                };
                if let Err(e) = process.status() {
                    let _ = writeln!(error_stream, "{e}");
                }
            }
            None => {
                let _ = writeln!(error_stream, "{cmd}: not found");
            }
        },
    }
}
