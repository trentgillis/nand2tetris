use std::{env, process};

use hack_assembler::assembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = assembler::cli_config::CliConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("Error occurred parsing arguments: {}", err);
        process::exit(1);
    });

    assembler::assemble(config).unwrap_or_else(|err| {
        eprintln!("Error occurred during assemble: {}", err);
        process::exit(1);
    });
}
