use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    thread::{self, available_parallelism},
};

use crate::{
    data::Data,
    fast_hash::FastHashBuilder,
    processing::{output_results, process_temperature},
};

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

                    process_chunk(buffer)
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

pub fn process_chunk(data: Vec<u8>) -> HashMap<Vec<u8>, Data, FastHashBuilder> {
    let mut temps: HashMap<Vec<u8>, Data, FastHashBuilder> =
        HashMap::with_capacity_and_hasher(512, FastHashBuilder);

    let mut start = 0;
    let mut station_key: &[u8] = b"";

    for (i, &b) in data.iter().enumerate() {
        if b == b';' {
            station_key = unsafe { data.get_unchecked(start..i) };
            start = i + 1;
        } else if b == b'\n' {
            let temperature = unsafe { data.get_unchecked(start..i) };
            let temp = process_temperature(temperature);

            temps
                .entry(station_key.to_vec())
                .and_modify(|temps| temps.update(temp))
                .or_insert_with(|| Data {
                    min: temp,
                    max: temp,
                    count: 1,
                    total: 1,
                });

            start = i + 1;
        }
    }

    temps
}

// samply record --windows-symbol-server https://msdl.microsoft.com/download/symbols --breakpad-symbol-server https://symbols.mozilla.org/try/ --windows-symbol-server https://chromium-browser-symsrv.commondatastorage.googleapis.com target/release/brc
