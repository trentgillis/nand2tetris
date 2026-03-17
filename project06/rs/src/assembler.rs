use std::{
    error::Error,
    fs,
    io::{self, BufRead},
};

pub mod cli_config;
mod code_gen;
mod parser;

pub fn assemble(cfg: cli_config::CliConfig) -> Result<(), Box<dyn Error>> {
    let assembler = Assembler::new();
    let file = fs::File::open(&cfg.file_name)?;

    // assembler.populate_labels();
    assembler.assemble(io::BufReader::new(file))?;

    Ok(())
}

struct Assembler {}

impl Assembler {
    fn new() -> Assembler {
        Assembler {}
    }

    fn assemble<R: BufRead>(&self, reader: R) -> Result<(), Box<dyn Error>> {
        for line in reader.lines() {
            let line = line?;

            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            match parser::instruction_type(&line) {
                parser::InstructionType::A => self.assemble_a_instruction(&line),
                parser::InstructionType::C => self.assemble_c_instruction(&line),
                parser::InstructionType::L => self.assemble_l_instruction(&line),
            }
        }

        Ok(())
    }

    fn assemble_a_instruction(&self, line: &str) {
        println!("A_INSTRUCTION: {line}");
    }

    fn assemble_c_instruction(&self, line: &str) {
        println!("C_INSTRUCTION: {line}");
    }

    fn assemble_l_instruction(&self, line: &str) {
        println!("L_INSTRUCTION: {line}");
    }
}

#[cfg(test)]
mod tests {}
