use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process;

const BUILT_IN_COMMANDS: &[&str] = &["exit", "echo", "type", "pwd"];
enum Command {
    Exit,
    Echo { arg: String },
    Type { arg: String },
    Pwd,
    Cd { arg: String },
    ExecutableOrNotFound { bin: String, args: String },
}

impl Command {
    fn from_input(input: &str) -> Command {
        let input = input.trim();
        if input == "exit" {
            return Self::Exit;
        }

        if input == "pwd" {
            return Self::Pwd;
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
        if input.starts_with("cd") {
            let arg = input.strip_prefix("cd").unwrap();
            return Self::Cd {
                arg: arg.trim().to_string(),
            };
        }
        let bin = input.split_whitespace().next().unwrap().to_string();
        let args = input[bin.len()..].to_string();
        return Self::ExecutableOrNotFound { bin, args };
    }
}

fn try_run_cmd(bin: &str, args: &str) -> std::io::Result<process::Child> {
    let mut cmd = process::Command::new(bin);
    if !args.is_empty() {
        cmd.args(args.split_whitespace());
    }
    cmd.spawn()
}

fn main() {
    'repl: loop {
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
                    continue 'repl;
                }

                let path_env = env::var_os("PATH");
                if let Some(paths) = path_env {
                    for folder in env::split_paths(&paths) {
                        let full_path = folder.join(&arg);
                        if full_path.exists() && full_path.is_file() {
                            if full_path.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                                println!("{} is {}", arg, full_path.display());
                                continue 'repl;
                            }
                        }
                    }
                }
                println!("{}: not found", arg);
            }
            Command::Pwd => println!("{}", env::current_dir().unwrap().display()),
            Command::Cd { arg } => {
                let path = env::current_dir().unwrap();
                let new_path = path.join(&arg);
                if new_path.exists() && new_path.is_dir() {
                    env::set_current_dir(&new_path).unwrap();
                } else {
                    println!("{}: No such file or directory", arg);
                }
            }
            Command::ExecutableOrNotFound { bin, args } => match try_run_cmd(&bin, &args) {
                Ok(mut child) => {
                    let _ = child.wait();
                }
                Err(_e) => println!("{}: command not found", input.trim()),
            },
        }
    }
}
