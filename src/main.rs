mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use crate::memory_mapped::memory_mapped;
use crate::buffered_reader::buffered_reader;

static FILE_PATH: &str = "data.txt";

fn main() {
    //memory_mapped(FILE_PATH);

    buffered_reader(FILE_PATH);
}
