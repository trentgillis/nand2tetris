use std::{collections::HashMap, error::Error, io::Write, sync::LazyLock};

static SEGMENT_MAPPINGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("local", "LCL"),
        ("argument", "ARG"),
        ("this", "THIS"),
        ("that", "THAT"),
    ])
});

static OPERATOR_MAPPINGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("add", "+"),
        ("sub", "-"),
        ("and", "&"),
        ("or", "|"),
        ("neg", "-"),
        ("not", "!"),
        ("eq", "JEQ"),
        ("lt", "JLT"),
        ("gt", "JGT"),
    ])
});

pub struct CodeWriter<W: Write> {
    output: W,
    num_labels: u32,
}

impl<W: Write> CodeWriter<W> {
    pub fn new(output: W) -> Self
    where
        W: Write,
    {
        CodeWriter {
            output,
            num_labels: 0,
        }
    }

    pub fn write_push(&mut self, segment: &str, index: &str) -> Result<(), Box<dyn Error>> {
        match segment {
            "constant" => {
                writeln!(self.output, "@{index}")?;
                writeln!(self.output, "D=A")?;
            }
            "static" => {
                // TODO: somename needs to be the program name
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

    pub fn write_arithmetic(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        match command {
            "add" | "sub" | "and" | "or" => {
                let op = OPERATOR_MAPPINGS.get(command).copied().unwrap();
                writeln!(self.output, "@SP")?;
                writeln!(self.output, "AM=M-1")?;
                writeln!(self.output, "D=M")?;
                writeln!(self.output, "A=A-1")?;
                writeln!(self.output, "M=M{op}D")?;
            }
            "neg" | "not" => {
                let op = OPERATOR_MAPPINGS.get(command).copied().unwrap();
                writeln!(self.output, "@SP")?;
                writeln!(self.output, "A=M-1")?;
                writeln!(self.output, "M={op}M")?;
            }
            "eq" | "gt" | "lt" => {
                self.num_labels += 1;
                let jmp = OPERATOR_MAPPINGS.get(command).copied().unwrap();
                // TODO: use the program name instead of todo
                let label = format!("{}.{}.{}", "todo", command.to_uppercase(), self.num_labels);
                let end_label = format!(
                    "{}.{}_END.{}",
                    "todo",
                    command.to_uppercase(),
                    self.num_labels
                );

                writeln!(self.output, "@SP")?;
                writeln!(self.output, "AM=M-1")?;
                writeln!(self.output, "D=M")?;
                writeln!(self.output, "A=A-1")?;
                writeln!(self.output, "D=M-D")?;
                writeln!(self.output, "@{label}")?;
                writeln!(self.output, "D;{jmp}")?;
                writeln!(self.output, "@SP")?;
                writeln!(self.output, "A=M-1")?;
                writeln!(self.output, "M=0")?;
                writeln!(self.output, "@{end_label}")?;
                writeln!(self.output, "0;JEQ")?;
                writeln!(self.output, "({label})")?;
                writeln!(self.output, "@SP")?;
                writeln!(self.output, "A=M-1")?;
                writeln!(self.output, "M=-1")?;
                writeln!(self.output, "({end_label})")?;
            }
            _ => {
                return Err(
                    format!("Error writing arithmetic, unknown command: {}", command).into(),
                );
            }
        }

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
