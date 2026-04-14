use std::{env, error::Error, process};

use vm_translator::vm_translator;

fn main() {
    if let Err(err) = run() {
        eprint!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cfg = vm_translator::cli_config::CliConfig::build(env::args())?;
    vm_translator::translate(cfg)?;
    Ok(())
}
