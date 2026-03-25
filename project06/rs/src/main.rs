use std::{env, process};

use hack_assembler::assembler;

fn main() {
    let config = assembler::cli_config::CliConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error occurred parsing arguments: {}", err);
        process::exit(1);
    });

    assembler::assemble(config).unwrap_or_else(|err| {
        eprintln!("Error occurred during assemble: {}", err);
        process::exit(1);
    });
}
