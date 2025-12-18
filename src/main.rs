#![feature(test)]
mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;
mod bench;

use std::time::Instant;

use crate::memory_mapped::memory_mapped;
use crate::buffered_reader::buffered_reader;

static FILE_PATH: &str = "D:/data.txt";

fn main() {
    let time = Instant::now();
    
    if cfg!(target_os = "macos") {
        buffered_reader(FILE_PATH);
    } else {
        memory_mapped::<32>(FILE_PATH);
    }

    println!("{:?}", time.elapsed());
}
