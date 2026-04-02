pub struct CliConfig {
    pub program_path: String,
}

impl CliConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let program_path = match args.next() {
            Some(path) => path,
            None => return Err("Program path for .vm file(s) to translate was not provided"),
        };

        Ok(CliConfig { program_path })
    }
}
