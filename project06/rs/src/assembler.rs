use std::{
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

use crate::assembler::parser::symbol;

pub mod cli_config;
mod code_gen;
mod parser;
mod symbol_table;

pub fn assemble(cfg: cli_config::CliConfig) -> Result<(), Box<dyn Error>> {
    let asm_file = fs::File::open(&cfg.file_name)?;
    let hack_file = fs::File::create(cfg.file_name.replacen(".asm", ".hack", 1))?;
    let mut assembler = Assembler::new(hack_file);

    assembler.populate_labels(io::BufReader::new(&asm_file))?;
    assembler.assemble(io::BufReader::new(&asm_file))?;

    Ok(())
}

struct Assembler<W: Write> {
    symbol_table: symbol_table::SymbolTable,
    output: W,
}

impl<W: Write> Assembler<W> {
    fn new(output: W) -> Self {
        Assembler {
            symbol_table: symbol_table::SymbolTable::new(),
            output,
        }
    }

    fn assemble<R>(&mut self, reader: R) -> Result<(), Box<dyn Error>>
    where
        R: BufRead,
    {
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            match parser::instruction_type(line) {
                parser::InstructionType::A => self.assemble_a_instruction(line)?,
                parser::InstructionType::C => self.assemble_c_instruction(line)?,
                parser::InstructionType::L => self.assemble_l_instruction(line)?,
            }
        }

        Ok(())
    }

    fn populate_labels<R>(&mut self, reader: R) -> Result<(), Box<dyn Error>>
    where
        R: BufRead,
    {
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            if parser::instruction_type(line) == parser::InstructionType::L {
                let label = symbol(line)?;
                self.symbol_table.insert(label);
            }
        }

        Ok(())
    }

    fn assemble_a_instruction(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let symbol_str = parser::symbol(line)?;
        let symbol_num: u32 = symbol_str.parse().unwrap_or_else(|_| {
            if !self.symbol_table.contains(symbol_str) {
                self.symbol_table.insert(symbol_str);
            }
            *self.symbol_table.get(symbol_str).unwrap()
        });

        let binary = format!("0{:015b}", symbol_num);
        writeln!(self.output, "{}", binary)?;

        Ok(())
    }

    fn assemble_c_instruction(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let dest = code_gen::dest(parser::dest(line)?);
        let comp = code_gen::comp(parser::comp(line)?);
        let jump = code_gen::jump(parser::jump(line)?);
        writeln!(self.output, "111{}{}{}", comp, dest, jump)?;

        Ok(())
    }

    fn assemble_l_instruction(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        writeln!(self.output, "L_INSTRUCTION: {line}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_assemble_no_lables() {
        let input = b"@2\nD=A\n@3\nD=D+A\n@0\nM=D";
        let expected = "0000000000000010\n1110110000010000\n0000000000000011\n1110000010010000\n0000000000000000\n1110001100001000\n";

        let mut assembler = Assembler::new(Vec::new());
        assembler.assemble(BufReader::new(&input[..])).unwrap();
        let output = String::from_utf8(assembler.output).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_populate_labels() {
        let input = b"(LOOP_1)\n(LOOP_2)\n";
        let mut assembler = Assembler::new(Vec::new());
        assembler
            .populate_labels(BufReader::new(&input[..]))
            .unwrap();
        assert!(assembler.symbol_table.entries.contains_key("LOOP_1"));
        assert!(assembler.symbol_table.entries.contains_key("LOOP_2"));
    }

    #[test]
    fn test_assembler_a_instruction_addr() {
        let mut assembler = Assembler::new(Vec::new());
        assembler
            .assemble_a_instruction("@3")
            .expect("Should assemble A instruction @3");
        let output = String::from_utf8(assembler.output).unwrap();
        assert_eq!(output, "0000000000000011\n");
    }

    #[test]
    fn test_assembler_a_instruction_var() {
        let mut assembler = Assembler::new(Vec::new());
        assembler
            .assemble_a_instruction("@myvar")
            .expect("Should assemble A instruction @myvar");
        let output = String::from_utf8(assembler.output).unwrap();
        assert_eq!(output, "0000000000010000\n");
    }
}
