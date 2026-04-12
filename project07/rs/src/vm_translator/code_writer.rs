use std::io::Write;

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
}
