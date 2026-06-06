mod completer;
mod executor;
mod lexer;
mod parser;

use crate::completer::ShellCompleter;
use crate::executor::execute_command;
use crate::lexer::LexerExt;
use crate::parser::parse_commands;
use rustyline::Editor;
use rustyline::error::ReadlineError;
#[allow(unused_imports)]
use std::io::{self, Write};

const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];
fn main() {
    let h = ShellCompleter {};
    let mut editor = Editor::new().expect("Editor error");
    editor.set_helper(Some(h));

    loop {
        let input = match editor.readline("$ ") {
            Ok(input) => input,
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(e) => {
                println!("shell error: {:?}", e);
                continue;
            }
        };

        for command in parse_commands(input.trim().tokenize()) {
            execute_command(command);
        }
    }
}
