use std::{
    fs,
    io::{self, BufRead},
};

pub mod cli_config;

pub fn assemble(cfg: cli_config::CliConfig) -> io::Result<()> {
    let file = fs::File::open(cfg.file_name)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{line}");
    }

    Ok(())
}
