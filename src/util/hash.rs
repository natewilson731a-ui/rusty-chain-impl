use std::io;
use blake2::{Blake2s256, Digest};


pub const HASH256_BYTES: usize = 32;

pub struct Hash256 {
    hasher: Blake2s256,
}

impl Hash256 {
    pub fn new() -> Hash256 {
        Hash256 {
            hasher: Blake2s256::new(),
        }
    }

    pub fn finalize(&mut self, buf: &mut [u8]) {
        let result = self.hasher.finalize_reset();
        buf.copy_from_slice(&result)
    }

    pub fn reset(&mut self) {
        self.hasher = Blake2s256::new();
    }
}

impl io::Write for Hash256 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
