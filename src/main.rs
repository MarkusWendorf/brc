mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use crate::buffered_reader::buffered_reader;
use crate::memory_mapped::memory_mapped;

static FILE_PATH: &str = "data.txt";

fn main() {
    if cfg!(target_os = "macos") {
        buffered_reader(FILE_PATH);
    } else {
        memory_mapped::<32>(FILE_PATH);
    }
}
