use std::{array::from_fn, fs::OpenOptions, thread};

use memmap2::MmapMut;

use crate::processing::{output_results, process_chunk};

pub fn memory_mapped(file_path: &str) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .unwrap();

    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };
    //mmap.advise(memmap2::Advice::Sequential).unwrap();

    let processed_chunks = thread::scope(|scope| {
        let chunk_count = 32;
        let mut indices: [std::ops::Range<usize>; 32] = from_fn::<_, 32, _>(|_| 0..1);

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

        let chunks = mmap.get_disjoint_mut(indices).unwrap();

        chunks
            .map(|chunk| scope.spawn(|| process_chunk(chunk)))
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>()
    });

    output_results(processed_chunks);
}
