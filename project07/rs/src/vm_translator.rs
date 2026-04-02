pub mod cli_config;

pub fn translate(cfg: cli_config::CliConfig) {
    println!("File path: {}", cfg.program_path);
}
