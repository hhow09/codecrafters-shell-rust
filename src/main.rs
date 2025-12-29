#[allow(unused_imports)]
use std::io::{self, Write};

const BUILT_IN_COMMANDS: &[&str] = &["echo", "exit", "type"];
enum Command {
    Exit,
    Echo { arg: String },
    Type { arg: String },
    NotFound,
}

impl Command {
    fn from_input(input: &str) -> Command {
        let input = input.trim();
        if input == "exit" {
            return Self::Exit;
        }

        if input.starts_with("echo") {
            let arg = input.strip_prefix("echo").unwrap();
            return Self::Echo {
                arg: arg.trim().to_string(),
            };
        }

        if input.starts_with("type") {
            let arg = input.strip_prefix("type").unwrap();
            return Self::Type {
                arg: arg.trim().to_string(),
            };
        }
        return Self::NotFound;
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = Command::from_input(&input);
        match command {
            Command::Exit => return,
            Command::Echo { arg } => println!("{}", arg),
            Command::Type { arg } => {
                if BUILT_IN_COMMANDS.contains(&arg.as_str()) {
                    println!("{} is a shell builtin", arg);
                } else {
                    println!("{}: not found", arg);
                }
            }
            Command::NotFound => println!("{}: command not found", input.trim()),
        }
    }
}
