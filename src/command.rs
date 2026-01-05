use crate::const_vals::*;
use std::io;
use std::io::Error;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ParseMode {
    SingleQuote,
    DoubleQuote,
    None,
}

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
            let args = Self::parse_args(arg).unwrap();
            return Self::Echo { args };
        }

        if input.starts_with("type ") {
            let arg = input.strip_prefix("type ").unwrap();
            let args = Self::parse_args(arg).unwrap();
            return Self::Type { args };
        }
        if input.starts_with("cd ") {
            let arg = input.strip_prefix("cd").unwrap();
            let args = Self::parse_args(arg).unwrap();
            return Self::Cd { args };
        }
        let bin = input.split(' ').next().unwrap().to_string();
        let args = Self::parse_args(&input[bin.len()..]).unwrap();
        return Self::ExecutableOrNotFound { bin, args };
    }

    fn parse_args(args_raw: &str) -> Result<Vec<String>, Error> {
        Self::parse_args_handle_quote(args_raw)
    }

    fn parse_args_handle_quote(args_raw: &str) -> Result<Vec<String>, Error> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut mode: ParseMode = ParseMode::None;
        let mut escaping = false;

        for ch in args_raw.chars() {
            match ch {
                SINGLE_QUOTE => match mode {
                    ParseMode::SingleQuote => match escaping {
                        true => {
                            current_arg.push(ch);
                            escaping = false;
                        }
                        false => {
                            mode = ParseMode::None;
                        }
                    },
                    ParseMode::None => match escaping {
                        true => {
                            current_arg.push(ch);
                            escaping = false;
                        }
                        false => {
                            mode = ParseMode::SingleQuote;
                        }
                    },
                    _ => current_arg.push(ch),
                },
                DOUBLE_QUOTE => match mode {
                    ParseMode::DoubleQuote => {
                        if !escaping {
                            mode = ParseMode::None;
                        } else {
                            current_arg.push(ch);
                            escaping = false;
                        }
                    }
                    ParseMode::None => {
                        if !escaping {
                            mode = ParseMode::DoubleQuote;
                        } else {
                            current_arg.push(ch);
                            escaping = false;
                        }
                    }
                    _ => current_arg.push(ch),
                },
                ESCAPE => match mode {
                    // escape only works in None mode
                    ParseMode::None => match escaping {
                        true => {
                            current_arg.push(ch);
                            escaping = false;
                        }
                        false => escaping = true,
                    },
                    _ => current_arg.push(ch),
                },
                _ => {
                    // if not in quote, not escaping, and whitespace, then is a new arg
                    if ch.is_whitespace() && mode == ParseMode::None && !escaping {
                        if !current_arg.is_empty() {
                            args.push(std::mem::take(&mut current_arg));
                        }
                    } else {
                        current_arg.push(ch);
                    }
                    if escaping {
                        escaping = false;
                    }
                }
            }
        }
        if !current_arg.is_empty() && mode != ParseMode::None {
            return Err(Error::new(io::ErrorKind::InvalidData, "Invalid input"));
        }
        if !current_arg.is_empty() {
            args.push(std::mem::take(&mut current_arg));
        }
        Ok(args)
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

    #[test]
    fn test_echo_with_double_quotes() {
        let input = "echo \"quz  hello\" \"bar\"";
        let command = Command::from_input(input);

        let expected_args = vec!["quz  hello".to_string(), "bar".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            }
        );
    }

    #[test]
    fn test_echo_double_quotes_wrapping_single_quotes() {
        let input = "echo \"shell's test\"";
        let command = Command::from_input(input);

        let expected_args = vec!["shell's test".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            }
        );
    }

    #[test]
    fn test_parse_args_with_escaping() {
        let input = "echo world\\ \\ \\ \\ \\ \\ script";
        let command = Command::from_input(input);

        let expected_args = vec!["world      script".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            },
            "should keep whitespace"
        );
    }

    #[test]
    fn test_parse_args_with_escaping_backslash() {
        let input = "echo hello\\\\world";
        let command = Command::from_input(input);

        let expected_args = vec!["hello\\world".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            },
            "should keep backslashes"
        );
    }

    #[test]
    fn test_parse_args_with_backslash_n() {
        let input = "echo test\\nexample";
        let command = Command::from_input(input);

        let expected_args = vec!["testnexample".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            },
            "should remove backslashes"
        );
    }

    #[test]
    fn test_parse_args_with_newline() {
        let input = "echo 'test\nexample'";
        let command = Command::from_input(input);

        let expected_args = vec!["test\nexample".to_string()];

        assert_eq!(
            command,
            Command::Echo {
                args: expected_args
            },
            "should preserve newlines in single quotes"
        );
    }
}
