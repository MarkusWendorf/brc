use std::{array::from_fn, collections::HashMap, fs::OpenOptions, thread};

use memmap2::MmapMut;

use crate::{data::Data, fast_hash::FastHashBuilder, processing::process_chunk};

pub fn memory_mapped(file_path: &str) -> Vec<HashMap<Vec<u8>, Data, FastHashBuilder>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .unwrap();

    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };
    //mmap.advise(memmap2::Advice::Sequential).unwrap();

    let chunk_count = 64;
    let mut indices: [std::ops::Range<usize>; 64] = from_fn::<_, 64, _>(|_| 0..1);

    let total_length = mmap.len();

    let chunk_size = (total_length / chunk_count) as usize;

    let mut offset: usize = 0;

    for i in 0..chunk_count {
        let chunk_start = offset;
        let mut chunk_end = offset + chunk_size;

        let mut chunk_end_pos = chunk_end;
        loop {
            match mmap.get(chunk_end_pos) {
                Some(character) if *character == b'\n' => {
                    chunk_end = chunk_end_pos;
                    break;
                }
                Some(_) => chunk_end_pos += 1,
                None => {
                    chunk_end = total_length;
                    break;
                }
            }
        }

        indices[i] = chunk_start..chunk_end;
        offset = chunk_end + 1;
    }

    thread::scope(|scope| {
        let chunks = mmap.get_disjoint_mut(indices).unwrap();

        let parts = chunks
            .map(|chunk| scope.spawn(|| process_chunk(Vec::from(chunk))))
            .into_iter();

        parts.map(|p| p.join().unwrap()).collect()
    })
}
