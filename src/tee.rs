use std::io::{Read, Result, Write};

pub struct TeeReader<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> TeeReader<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        TeeReader {
            reader: reader,
            writer: writer,
        }
    }
}

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.reader.read(buf)?;
        self.writer.write_all(&buf)?;
        Ok(n)
    }
}
