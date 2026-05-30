mod executor;
mod lexer;
mod parser;

use crate::executor::execute_command;
use crate::lexer::LexerExt;
use crate::parser::parse_commands;
#[allow(unused_imports)]
use std::io::{self, Write};

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
