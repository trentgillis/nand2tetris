use std::{env, process};

use vm_translator::vm_translator;

fn main() {
    let cli_config =
        vm_translator::cli_config::CliConfig::build(env::args()).unwrap_or_else(|err| {
            eprint!("Error occured parsing arugments: {}", err);
            process::exit(0);
        });

    vm_translator::translate(cli_config);
}
