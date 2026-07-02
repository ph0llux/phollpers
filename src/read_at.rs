// - STD
use std::io::{Read};

// - internal
use crate::traits::read_at::ReadAt;
pub struct ReadAtCursor<'a, R: ReadAt + ?Sized> {
    data: &'a R,
    position: u64,
}

impl<'a, R: ReadAt + ?Sized> ReadAtCursor<'a, R> {
    pub fn new(data: &'a R, offset: u64) -> Self {
        Self {
            data,
            position: offset,
        }
    }
}

impl<R: ReadAt + ?Sized> Read for ReadAtCursor<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.data.read_at(buf, self.position)?;
        self.position += bytes_read as u64;
        Ok(bytes_read)
    }
}