mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use crate::memory_mapped::memory_mapped;

static FILE_PATH: &str = "data.txt";

fn main() {
    memory_mapped(FILE_PATH);
}
