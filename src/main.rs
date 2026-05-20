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

        let mut parts = command.split_whitespace();

        match parts.next() {
            Some("exit") => break,
            Some("echo") => {
                parts.for_each(|arg| print!("{arg} "));
                println!();
            }
            Some(cmd) => println!("{cmd}: command not found"),
            None => {}
        }
    }
}
