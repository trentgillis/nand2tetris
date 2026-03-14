pub struct CliConfig {
    pub file_name: String,
}

impl CliConfig {
    pub fn build(args: &[String]) -> Result<CliConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_name = args[1].clone();

        Ok(CliConfig { file_name })
    }
}
