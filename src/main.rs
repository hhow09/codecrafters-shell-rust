#[allow(unused_imports)]
use std::io::{self, Write};

type Command = &'static str;
const ECHO: Command = "echo";
const EXIT: Command = "exit";
const TYPE: Command = "type";

fn is_builtin(command: &str) -> bool {
    matches!(command.trim(), ECHO | EXIT | TYPE)
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command.trim() == EXIT {
            return;
        }
        if command.starts_with(ECHO) {
            let arg = command.strip_prefix(ECHO).unwrap();
            println!("{}", arg.trim());
        } else if command.starts_with(TYPE) {
            let arg = command.strip_prefix(TYPE).unwrap();
            if is_builtin(arg) {
                println!("{} is a shell builtin", arg.trim());
            } else {
                println!("{}: not found", arg.trim())
            }
        } else {
            println!("{}: command not found", command.trim())
        }
    }
}
