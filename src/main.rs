mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use crate::buffered_reader::buffered_reader;
use crate::memory_mapped::memory_mapped;

static FILE_PATH: &str = "data.txt";

fn main() {
    memory_mapped::<32>(FILE_PATH);

    //buffered_reader(FILE_PATH);
}
