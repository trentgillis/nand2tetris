use std::{
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

pub mod cli_config;
mod code_gen;
mod parser;

pub fn assemble(cfg: cli_config::CliConfig) -> Result<(), Box<dyn Error>> {
    let asm_file = fs::File::open(&cfg.file_name)?;
    let hack_file = fs::File::create(cfg.file_name.replacen(".asm", ".hack", 1))?;
    let mut assembler = Assembler::new(hack_file);

    // assembler.populate_labels();
    assembler.assemble(io::BufReader::new(asm_file))?;

    Ok(())
}

struct Assembler<W: Write> {
    output: W,
}

impl<W: Write> Assembler<W> {
    fn new(output: W) -> Self {
        Assembler { output }
    }

    fn assemble<R: BufRead>(&mut self, reader: R) -> Result<(), Box<dyn Error>> {
        for line in reader.lines() {
            let line = line?;

            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            match parser::instruction_type(&line) {
                parser::InstructionType::A => self.assemble_a_instruction(&line)?,
                parser::InstructionType::C => self.assemble_c_instruction(&line),
                parser::InstructionType::L => self.assemble_l_instruction(&line),
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

    fn assemble_c_instruction(&mut self, line: &str) {
        writeln!(self.output, "C_INSTRUCTION: {line}");
    }

    fn assemble_l_instruction(&mut self, line: &str) {
        writeln!(self.output, "L_INSTRUCTION: {line}");
    }
}

#[cfg(test)]
mod tests {}
