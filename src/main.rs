#![feature(test)]
mod bench;
mod buffered_reader;
mod data;
mod memory_mapped;
mod processing;

use std::time::Instant;

static FILE_PATH: &str = "measurements.txt";

fn main() {
    let time = Instant::now();

    //buffered_reader::buffered_reader(FILE_PATH);
    memory_mapped::memory_mapped::<10>(FILE_PATH);

    println!("\n{:?}", time.elapsed());
}
