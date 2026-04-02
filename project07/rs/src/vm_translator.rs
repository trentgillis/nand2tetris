use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub mod cli_config;

pub fn translate(cfg: cli_config::CliConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&cfg.program_path);
    if !path.exists() {
        return Err("The supplied program path does not exist.".into());
    }

    let vm_files = get_vm_files(&cfg.program_path);
    for file in vm_files? {
        translate_file(file)?;
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

fn translate_file(path: PathBuf) -> Result<(), &'static str> {
    println!("Path: {:?}", path);
    Ok(())
}
