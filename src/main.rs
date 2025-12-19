#![feature(test)]
mod bench;
mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use std::time::Instant;

use crate::buffered_reader::buffered_reader;
use crate::memory_mapped::memory_mapped;

static FILE_PATH: &str = "data.txt";

fn main() {
    let time = Instant::now();

    // if cfg!(target_os = "macos") {
    // buffered_reader(FILE_PATH);
    // } else {
    memory_mapped::<10>(FILE_PATH);
    // }

    println!("{:?}", time.elapsed());
}
