pub use crc::crc32::Digest;
use std::hash::Hasher;
use std::io;

pub struct DigestWriter<'a>(&'a mut Digest);

impl<'a> DigestWriter<'a> {
    pub fn new(digest: &'a mut Digest) -> Self {
        Self(digest)
    }
}

impl<'a> io::Write for DigestWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write(buf);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
