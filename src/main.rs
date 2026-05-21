#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the input");

        let mut args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }
        let command = args.remove(0);

        match command {
            "exit" => break,
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                if ["exit", "echo", "type"].contains(&args[0]) {
                    println!("{} is a shell builtin", args[0]);
                } else {
                    println!("{}: not found", args[0]);
                }
            }
            cmd => println!("{cmd}: not found"),
        }
    }
}
