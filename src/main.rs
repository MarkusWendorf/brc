#![feature(test)]
mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use crate::memory_mapped::memory_mapped;

static FILE_PATH: &str = "data.txt";

fn main() {
    //buffered_reader(FILE_PATH);
    // Specify number of threads best suited for your machine
    memory_mapped::<10>(FILE_PATH);
}
