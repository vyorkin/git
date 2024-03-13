use std::io::{self, Read};

pub struct LimitReader<R> {
    reader: R,
    limit: usize,
}

impl<R> LimitReader<R> {
    pub fn new(reader: R, limit: usize) -> Self {
        Self { reader, limit }
    }
}

impl<R> Read for LimitReader<R>
where
    R: Read,
{
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() > self.limit {
            buf = &mut buf[..self.limit + 1];
        }
        let bytes_read = self.reader.read(buf)?;
        if bytes_read > self.limit {
            return Err(io::Error::new(io::ErrorKind::Other, "too many bytes"));
        }
        self.limit -= bytes_read;

        Ok(bytes_read)
    }
}
