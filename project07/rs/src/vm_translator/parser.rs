use std::error::Error;

const ARITHMETIC_COMMANDS: [&str; 3] = ["add", "sub", "neg"];
const LOGICAL_COMMANDS: [&str; 6] = ["eq", "gt", "lt", "and", "or", "not"];

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Arithmetic(&'a str),
    Push { segment: &'a str, index: &'a str },
    Pop { segment: &'a str, index: &'a str },
}

pub fn parse(command: &str) -> Result<Command<'_>, Box<dyn Error>> {
    let mut parts = command.split_whitespace();
    match parts.next().ok_or("Unable to process empty VM command")? {
        "push" => Ok(Command::Push {
            segment: parts.next().ok_or("Segment not provided to push")?,
            index: parts.next().ok_or("Index not provided to push")?,
        }),
        "pop" => Ok(Command::Pop {
            segment: parts.next().ok_or("Segment not provided to pop")?,
            index: parts.next().ok_or("Index not provided to pop")?,
        }),
        cmd if is_arithmetic_logical(command) => Ok(Command::Arithmetic(cmd)),
        cmd => Err(format!("Unknown command: {cmd}").into()),
    }
}

fn is_arithmetic_logical(command: &str) -> bool {
    ARITHMETIC_COMMANDS.contains(&command) || LOGICAL_COMMANDS.contains(&command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arithmetic_logical_command_type() {
        for command in ARITHMETIC_COMMANDS {
            let cmd = parse(command).unwrap();
            assert_eq!(cmd, Command::Arithmetic(command));
        }
        for command in LOGICAL_COMMANDS {
            let cmd = parse(command).unwrap();
            assert_eq!(cmd, Command::Arithmetic(command));
        }
    }
    #[test]
    fn test_push_command_type() {
        let cmd = parse("push constant 0").unwrap();
        assert_eq!(
            cmd,
            Command::Push {
                segment: "constant",
                index: "0"
            }
        );
    }
    #[test]
    fn test_pop_command_type() {
        let cmd = parse("pop local 0").unwrap();
        assert_eq!(
            cmd,
            Command::Pop {
                segment: "local",
                index: "0"
            }
        );
    }
}
