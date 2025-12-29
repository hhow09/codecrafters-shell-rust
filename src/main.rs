#[allow(unused_imports)]
use std::io::{self, Write};

const ECHO: &str = "echo";
const EXIT: &str = "exit";

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
        } else {
            println!("{}: command not found", command.trim())
        }
    }
}
