use std::hash::{BuildHasher, Hasher};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Default)]
pub struct FastHasher {
    hash: u64,
}

impl Hasher for FastHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.hash = xxh3_64(bytes);
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

#[derive(Clone, Copy, Default)]
pub struct FastHashBuilder;

impl BuildHasher for FastHashBuilder {
    type Hasher = FastHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FastHasher { hash: 0 }
    }
}
