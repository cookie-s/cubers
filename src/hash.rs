pub use sha2::{Digest, Sha256};
use std::io;

pub struct DigestWriter<'a>(&'a mut Sha256);

impl<'a> DigestWriter<'a> {
    pub fn new(digest: &'a mut Sha256) -> Self {
        Self(digest)
    }
}

impl<'a> io::Write for DigestWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.update(buf);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
