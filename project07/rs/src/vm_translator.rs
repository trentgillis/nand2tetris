use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

pub mod cli_config;

mod code_writer;
mod parser;

pub fn translate(cfg: cli_config::CliConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&cfg.program_path);
    if !path.exists() {
        return Err("The supplied program path does not exist.".into());
    }

    let output_path = if path.is_dir() {
        path.join(
            path.file_name()
                .ok_or("Unable to get output_path from input")?,
        )
        .with_extension("asm")
    } else {
        path.with_extension("asm")
    };
    let output_file = fs::File::create(output_path)?;

    let vm_file_paths = get_vm_files(&cfg.program_path);
    let mut vm_translator = VmTranslator::new(output_file);
    for path in vm_file_paths? {
        let file = fs::File::open(path)?;
        vm_translator.translate(file)?;
    }

    Ok(())
}

fn get_vm_files(program_path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let path = Path::new(program_path);
    if path.is_dir() {
        Ok(fs::read_dir(program_path)?
            .filter_map(|path| path.ok())
            .filter(|path| path.path().extension() == Some("vm".as_ref()))
            .map(|path| path.path())
            .collect())
    } else if path.extension() == Some("vm".as_ref()) {
        Ok(vec![path.to_path_buf()])
    } else {
        Err("Path was not a .vm file or a directory containing at least one .vm file".into())
    }
}

struct VmTranslator<W: Write> {
    code_writer: code_writer::CodeWriter<W>,
}

impl<W: Write> VmTranslator<W> {
    fn new(writer: W) -> Self {
        let code_writer = code_writer::CodeWriter::new(writer);
        VmTranslator { code_writer }
    }

    fn translate<R>(&mut self, r: R) -> Result<(), Box<dyn Error>>
    where
        R: Read,
    {
        let reader = BufReader::new(r);
        for line in reader.lines() {
            let line = line?;

            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            match parser::command_type(&line)? {
                parser::CommandType::Push => self.translate_push(&line)?,
                parser::CommandType::Pop => self.translate_push(&line)?,
                parser::CommandType::Arithmetic => self.translate_logical_arithemtic(&line)?,
            }
        }

        Ok(())
    }

    fn translate_push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let segment = parser::arg_1(line);
        let index = parser::arg_2(line);

        self.code_writer.write_push(segment, index)?;
        Ok(())
    }

    fn translate_pop(&self, line: &str) -> Result<(), Box<dyn Error>> {
        // noop
        Ok(())
    }

    fn translate_logical_arithemtic(&self, line: &str) -> Result<(), Box<dyn Error>> {
        // noop
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod vm_translator_test {
        mod translate_test {
            use super::super::super::*;

            #[test]
            fn test_translate_push() {}
            #[test]
            fn test_translate_pop() {}
            #[test]
            fn test_translate_arithemtic() {}
            #[test]
            fn test_translate_logical() {}
        }
    }
}
