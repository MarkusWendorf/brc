use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    thread::{self, available_parallelism},
};

use crate::processing::{output_results, process_chunk};

pub fn buffered_reader(file_path: &str) {
    thread::scope(|scope| {
        let file = OpenOptions::new().read(true).open(file_path).unwrap();
        let mut reader = BufReader::new(file);

        let total_length = reader.get_ref().metadata().unwrap().len() as usize;

        let chunk_count = available_parallelism().unwrap().get() * 2;
        let chunk_size = total_length / chunk_count;

        let mut indices: Vec<std::ops::Range<usize>> = Vec::new();
        let mut offset: usize = 0;

        for _ in 0..chunk_count {
            let chunk_start = offset;
            let mut chunk_end = offset + chunk_size;

            if chunk_end >= total_length {
                chunk_end = total_length;
            } else {
                reader.seek(SeekFrom::Start(chunk_end as u64)).unwrap();
                let mut buffer = Vec::new();

                chunk_end = match reader.read_until(b'\n', &mut buffer) {
                    Ok(_) => reader.stream_position().unwrap() as usize,
                    _ => total_length,
                };
            }

            indices.push(chunk_start..chunk_end);
            offset = chunk_end;
        }

        let handles = indices
            .into_iter()
            .map(|range| {
                scope.spawn(move || {
                    let file = OpenOptions::new().read(true).open(file_path).unwrap();
                    let mut reader = BufReader::new(file);
                    reader.seek(SeekFrom::Start(range.start as u64)).unwrap();

                    let mut buffer = vec![0u8; range.len()];
                    reader.read_exact(&mut buffer).unwrap();

                    process_chunk(&buffer, |k| k.to_vec())
                })
            })
            .collect::<Vec<_>>();

        let processed_chunks = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>();

        output_results(processed_chunks);
    });
}

// samply record --windows-symbol-server https://msdl.microsoft.com/download/symbols --breakpad-symbol-server https://symbols.mozilla.org/try/ --windows-symbol-server https://chromium-browser-symsrv.commondatastorage.googleapis.com target/release/brc
