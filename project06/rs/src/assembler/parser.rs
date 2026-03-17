#[derive(PartialEq, Debug)]
pub enum InstructionType {
    A,
    C,
    L,
}

pub fn instruction_type(line: &str) -> InstructionType {
    if line.starts_with("@") {
        InstructionType::A
    } else if line.starts_with("(") && line.ends_with(")") {
        InstructionType::L
    } else {
        InstructionType::C
    }
}

pub fn symbol(line: &str) -> &str {
    if instruction_type(line) == InstructionType::A {
        line.strip_prefix("@").unwrap()
    } else {
        line.strip_prefix("(").unwrap().strip_suffix(")").unwrap()
    }
}

pub fn dest(line: &str) -> Result<Option<&str>, String> {
    if instruction_type(line) != InstructionType::C {
        return Err(format!("dest() only callable on C instructions: {line}"));
    }

    let parts: Vec<&str> = line.split("=").collect();
    if parts.len() < 2 {
        return Ok(None);
    }

    Ok(Some(parts[0]))
}

pub fn jump(line: &str) -> Result<Option<&str>, String> {
    if instruction_type(line) != InstructionType::C {
        return Err(format!("jump() only callable on C instructions: {line}"));
    }

    let parts: Vec<&str> = line.split(";").collect();
    if parts.len() < 2 {
        return Ok(None);
    }

    Ok(Some(parts[1]))
}

#[cfg(test)]
mod tests {
    mod instruction_type {
        use super::super::*;

        #[test]
        fn test_instruction_type_a_instruction() {
            let inst_type = instruction_type("@aaa");
            assert_eq!(inst_type, InstructionType::A);
        }

        #[test]
        fn test_instruction_type_c_instruction() {
            let inst_type = instruction_type("dest=comp;jump");
            assert_eq!(inst_type, InstructionType::C);
        }

        #[test]
        fn test_instruction_type_l_instruction() {
            let inst_type = instruction_type("(aaa)");
            assert_eq!(inst_type, InstructionType::L);
        }
    }

    mod symbol {
        use super::super::*;

        #[test]
        fn test_symbol_a_instruction() {
            assert_eq!(symbol("@15"), "15");
            assert_eq!(symbol("@myvar"), "myvar");
        }

        #[test]
        fn test_symbol_l_instruction() {
            assert_eq!(symbol("(LOOP)"), "LOOP");
        }
    }

    mod dest {
        use super::super::*;

        #[test]
        fn test_dest_with_dest() {
            let dest = dest("M=D");
            assert!(dest.is_ok());
            assert!(dest.clone().unwrap().is_some());
            assert_eq!(dest.unwrap().unwrap(), "M");
        }

        #[test]
        fn test_dest_a_instruction() {
            let dest = dest("@myvar");
            assert!(dest.is_err());
        }

        #[test]
        fn test_dest_without_dest() {
            let dest = dest("0;JEQ");
            assert!(dest.is_ok());
            assert!(dest.unwrap().is_none());
        }
    }

    mod jump {
        use super::super::*;

        #[test]
        fn test_jump_with_jump() {
            let jump = jump("0;JEQ");
            assert!(jump.is_ok());
            assert!(jump.clone().unwrap().is_some());
            assert_eq!(jump.unwrap().unwrap(), "JEQ");
        }

        #[test]
        fn test_jump_a_instruction() {
            let jump = jump("@myvar");
            assert!(jump.is_err());
        }

        #[test]
        fn test_jump_without_jump() {
            let jump = jump("M=D");
            assert!(jump.is_ok());
            assert!(jump.unwrap().is_none());
        }
    }
}
