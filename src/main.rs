use codecrafters_shell::command::Command;
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process;

const BUILT_IN_COMMANDS: &[&str] = &["exit", "echo", "type", "pwd"];

fn try_run_cmd(bin: &str, args: &Vec<String>) -> std::io::Result<process::Child> {
    let mut cmd = process::Command::new(bin);
    if !args.is_empty() {
        cmd.args(args);
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
            Command::Echo { args } => println!("{}", args.join(" ")),
            Command::Type { args } => {
                let cmd = args[0].as_str();
                if BUILT_IN_COMMANDS.contains(&cmd) {
                    println!("{} is a shell builtin", cmd);
                    continue 'repl;
                }

                let path_env = env::var_os("PATH");
                if let Some(paths) = path_env {
                    for folder in env::split_paths(&paths) {
                        let full_path = folder.join(&cmd);
                        if full_path.exists() && full_path.is_file() {
                            if full_path.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                                println!("{} is {}", cmd, full_path.display());
                                continue 'repl;
                            }
                        }
                    }
                }
                println!("{}: not found", cmd);
            }
            Command::Pwd => println!("{}", env::current_dir().unwrap().display()),
            Command::Cd { args } => {
                let arg = args[0].as_str();
                if arg == "~" {
                    let home_dir = env::var("HOME").unwrap();
                    if home_dir == "" {
                        println!("cd: HOME not set");
                        continue 'repl;
                    }
                    env::set_current_dir(&home_dir).unwrap();
                    continue 'repl;
                }
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
