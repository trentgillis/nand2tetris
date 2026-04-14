use std::error::Error;

const ARITHMETIC_COMMANDS: [&str; 3] = ["add", "sub", "neg"];
const LOGICAL_COMMANDS: [&str; 6] = ["eq", "gt", "lt", "and", "or", "not"];

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
}

pub fn command_type(command: &str) -> Result<CommandType, Box<dyn Error>> {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let command_type = command_parts[0];

    match command_type {
        "push" => Ok(CommandType::Push),
        "pop" => Ok(CommandType::Pop),
        _ => {
            if is_arithmetic_logical(command) {
                return Ok(CommandType::Arithmetic);
            }
            Err(format!("Invalid command type: {}", command_type).into())
        }
    }
}

pub fn arg_1(command: &str) -> &str {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if is_arithmetic_logical(command) {
        return parts.first().copied().unwrap_or("");
    }

    parts.get(1).copied().unwrap_or("")
}

pub fn arg_2(command: &str) -> &str {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 3 {
        return "";
    }

    parts.get(2).copied().unwrap_or("")
}

fn is_arithmetic_logical(command: &str) -> bool {
    ARITHMETIC_COMMANDS.contains(&command) || LOGICAL_COMMANDS.contains(&command)
}

#[cfg(test)]
mod tests {
    mod command_type_test {
        use super::super::*;

        #[test]
        fn test_arithmetic_logical_command_type() {
            for command in ARITHMETIC_COMMANDS {
                let cmd_type = command_type(command).unwrap();
                assert_eq!(cmd_type, CommandType::Arithmetic);
            }
            for command in LOGICAL_COMMANDS {
                let cmd_type = command_type(command).unwrap();
                assert_eq!(cmd_type, CommandType::Arithmetic);
            }
        }
        #[test]
        fn test_push_command_type() {
            let cmd_type = command_type("push constant 0").unwrap();
            assert_eq!(cmd_type, CommandType::Push);
        }
        #[test]
        fn test_pop_command_type() {
            let cmd_type = command_type("pop local 0").unwrap();
            assert_eq!(cmd_type, CommandType::Pop);
        }
    }
    mod arg_1_test {
        use super::super::*;

        #[test]
        fn test_arg_1_push_pop() {
            let mut arg1 = arg_1("push constant 0");
            assert_eq!(arg1, "constant");
            arg1 = arg_1("pop local 0");
            assert_eq!(arg1, "local");
        }
        #[test]
        fn test_arg_1_logical_arithmetic() {
            let mut arg1 = arg_1("add");
            assert_eq!(arg1, "add");
            arg1 = arg_1("gt");
            assert_eq!(arg1, "gt");
        }
    }
    mod arg_2_test {
        use super::super::*;

        #[test]
        fn test_arg_1_logical_arithmetic() {
            let arg2 = arg_2("add");
            assert!(arg2.is_empty());
        }
        #[test]
        fn test_arg_2_push_pop() {
            let mut arg2 = arg_2("push constant 0");
            assert_eq!(arg2, "0");
            arg2 = arg_2("pop local 3");
            assert_eq!(arg2, "3");
        }
    }
}
