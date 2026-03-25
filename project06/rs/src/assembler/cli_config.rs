pub struct CliConfig {
    pub file_name: String,
}

impl CliConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<CliConfig, &'static str> {
        args.next();

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Path for .asm file to assemble was not passed."),
        };

        Ok(CliConfig { file_name })
    }
}
