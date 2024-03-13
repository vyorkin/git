use std::io::Write;

use sha1::{digest::Update, Sha1};

pub struct HashWriter<W> {
    pub writer: W,
    pub hasher: Sha1,
}

impl<W> HashWriter<W> {
    pub fn new(writer: W, hasher: Sha1) -> Self {
        HashWriter { writer, hasher }
    }
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_written = self.writer.write(buf)?;
        self.hasher.update(&buf[..bytes_written]);
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
