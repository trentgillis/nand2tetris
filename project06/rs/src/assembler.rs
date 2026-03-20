use std::{
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

pub mod cli_config;
mod code_gen;
mod parser;
mod symbol_table;

pub fn assemble(cfg: cli_config::CliConfig) -> Result<(), Box<dyn Error>> {
    let asm_file = fs::File::open(&cfg.file_name)?;
    let hack_file = fs::File::create(cfg.file_name.replacen(".asm", ".hack", 1))?;
    let mut assembler = Assembler::new(hack_file);

    // assembler.populate_labels();
    assembler.assemble(io::BufReader::new(asm_file))?;

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

    fn assemble<R: BufRead>(&mut self, reader: R) -> Result<(), Box<dyn Error>> {
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

    fn assemble_a_instruction(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let symbol: u16 = parser::symbol(line)?.parse().unwrap();
        let binary = format!("0{:015b}", symbol);
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
}
