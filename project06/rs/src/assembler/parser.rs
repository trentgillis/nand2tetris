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

pub fn dest(line: &str) -> Result<&str, String> {
    let parts: Vec<&str> = line.split("=").collect();
    if parts.len() < 2 {
        return Err(format!("Failed to get dest at line: {line}"));
    }

    Ok(parts[0])
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
            let asm = "M=D";
            let dest = dest(asm);
            assert!(dest.is_ok());
            assert_eq!(dest.unwrap(), "M");
        }

        #[test]
        fn test_dest_without_dest() {
            let asm = "0;JEQ";
            let dest = dest(asm);
            assert!(dest.is_err());
        }
    }
}
