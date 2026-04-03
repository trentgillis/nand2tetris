use std::error::Error;

const ARITHMETIC_COMMANDS: [&str; 3] = ["add", "sub", "neg"];
const LOGICAL_COMMANDS: [&str; 6] = ["eq", "gt", "lt", "and", "or", "not"];

#[derive(Debug, PartialEq)]
enum CommandType {
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

fn is_arithmetic_logical(command: &str) -> bool {
    ARITHMETIC_COMMANDS.contains(&command) || LOGICAL_COMMANDS.contains(&command)
}

#[cfg(test)]
mod tests {
    mod commmand_type_test {
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
}
