use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

pub mod cli_config;

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
    output: W,
}

impl<W: Write> VmTranslator<W> {
    fn new(writer: W) -> Self {
        VmTranslator { output: writer }
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
                parser::CommandType::Arithmetic => writeln!(self.output, "Arithmetic: {}", line)?,
                parser::CommandType::Push => writeln!(self.output, "Push: {}", line)?,
                parser::CommandType::Pop => writeln!(self.output, "Pop: {}", line)?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
