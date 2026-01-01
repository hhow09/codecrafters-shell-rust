#[derive(Debug, PartialEq)]
pub enum Command {
    Exit,
    Echo { args: Vec<String> },
    Type { args: Vec<String> },
    Pwd,
    Cd { args: Vec<String> },
    ExecutableOrNotFound { bin: String, args: Vec<String> },
}

impl Command {
    pub fn from_input(input: &str) -> Command {
        let input = input.trim();
        if input == "exit" {
            return Self::Exit;
        }

        if input == "pwd" {
            return Self::Pwd;
        }

        if input.starts_with("echo ") {
            let arg = input.strip_prefix("echo ").unwrap();
            let args = Self::parse_args(arg);
            return Self::Echo { args };
        }

        if input.starts_with("type ") {
            let arg = input.strip_prefix("type ").unwrap();
            let args = Self::parse_args(arg);
            return Self::Type { args };
        }
        if input.starts_with("cd ") {
            let arg = input.strip_prefix("cd").unwrap();
            let args = Self::parse_args(arg);
            return Self::Cd { args };
        }
        let bin = input.split(' ').next().unwrap().to_string();
        let args = Self::parse_args(&input[bin.len()..]);
        return Self::ExecutableOrNotFound { bin, args };
    }

    fn parse_args(args_raw: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut in_single_quote = false;
        let mut in_arg = false;

        for c in args_raw.chars() {
            if in_single_quote {
                if c == '\'' {
                    in_single_quote = false;
                } else {
                    current_arg.push(c);
                }
            } else {
                if c == '\'' {
                    in_single_quote = true;
                    in_arg = true;
                } else if c.is_whitespace() {
                    if in_arg {
                        args.push(current_arg.clone());
                        current_arg.clear();
                        in_arg = false;
                    }
                } else {
                    current_arg.push(c);
                    in_arg = true;
                }
            }
        }

        if in_arg {
            args.push(current_arg);
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_with_single_quotes_and_concatenation() {
        let input = "echo 'script     hello' 'example''world' test''shell";
        let command = Command::from_input(input);

        let expected_args = vec![
            "script     hello".to_string(),
            "exampleworld".to_string(),
            "testshell".to_string(),
        ];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            }
        );
    }

    #[test]
    fn test_basic_echo() {
        let input = "echo hello world";
        let command = Command::from_input(input);

        let expected_args = vec!["hello".to_string(), "world".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            }
        );
    }

    #[test]
    fn test_echo_single_quotes_spaces() {
        let input = "echo 'hello   world' 'foo'";
        let command = Command::from_input(input);

        let expected_args = vec!["hello   world".to_string(), "foo".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            }
        );
    }

    #[test]
    fn test_parse_args_with_empty_quotes() {
        let input = "echo 'a''b'";
        let command = Command::from_input(input);

        let expected_args = vec!["ab".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            }
        );
    }
}
