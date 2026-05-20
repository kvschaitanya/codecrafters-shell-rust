#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Couldn't read the input");

        command = command.trim().to_string();

        if command == "exit" {
            break;
        }

        let mut cmdline = command.split_whitespace().peekable();

        if *cmdline.peek().unwrap() == "echo" {
            cmdline.next();
            cmdline.for_each(|arg| print!("{arg} "));
            println!();
        } else {
            println!("{}: command not found", command);
        }
    }
}
