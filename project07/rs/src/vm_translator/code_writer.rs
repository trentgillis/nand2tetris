use std::{collections::HashMap, error::Error, io::Write, sync::LazyLock};

static SEGMENT_MAPPINGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("local", "LCL"),
        ("argument", "ARG"),
        ("this", "THIS"),
        ("that", "THAT"),
    ])
});

pub struct CodeWriter<W: Write> {
    output: W,
}

impl<W: Write> CodeWriter<W> {
    pub fn new(output: W) -> Self
    where
        W: Write,
    {
        CodeWriter { output }
    }

    pub fn write_push(&mut self, segment: &str, index: &str) -> Result<(), Box<dyn Error>> {
        match segment {
            "constant" => {
                writeln!(self.output, "@{index}")?;
                writeln!(self.output, "D=A")?;
            }
            "static" => {
                writeln!(self.output, "@somename.{index}")?;
                writeln!(self.output, "D=M")?;
            }
            "temp" | "pointer" => {
                let base_address = if segment.eq("pointer") { "3" } else { "5" };
                writeln!(self.output, "@{base_address}")?; // address of the temp base segment
                writeln!(self.output, "D=A")?;
                writeln!(self.output, "@{index}")?;
                writeln!(self.output, "A=D+A")?;
                writeln!(self.output, "D=M")?;
            }
            "local" | "argument" | "this" | "that" => {
                let segment_var = SEGMENT_MAPPINGS.get(segment).copied().unwrap_or("");
                writeln!(self.output, "@{segment_var}")?; // address of the temp base segment
                writeln!(self.output, "D=M")?;
                writeln!(self.output, "@{index}")?;
                writeln!(self.output, "A=D+A")?;
                writeln!(self.output, "D=M")?;
            }
            _ => return Err(format!("Error writing push, unknown segment: {}", segment).into()),
        }

        self.write_increment_sp()?;
        Ok(())
    }

    // Writes D to the address stored at @SP and increments @SP
    fn write_increment_sp(&mut self) -> Result<(), Box<dyn Error>> {
        writeln!(self.output, "@SP")?;
        writeln!(self.output, "A=M")?;
        writeln!(self.output, "M=D")?;
        writeln!(self.output, "@SP")?;
        writeln!(self.output, "M=M+1")?;
        Ok(())
    }
}
