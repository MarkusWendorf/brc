use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Seek, SeekFrom},
    os::unix::fs::FileExt,
    thread::{self},
};
use memchr::memchr;

use rapidhash::{HashMapExt, RapidHashMap};

use crate::{
    data::Data,
    processing::{output_results, process_temperature},
};

pub fn buffered_reader(file_path: &str) {
    thread::scope(|scope| {
        let file = OpenOptions::new().read(true).open(file_path).unwrap();

        let mut indices: Vec<std::ops::Range<usize>> = Vec::new();

        let mut reader = BufReader::new(file);

        let total_length = reader.get_ref().metadata().unwrap().len() as usize;

        let chunk_count = 32; //available_parallelism().unwrap().get() * 2;
        let chunk_size = total_length / chunk_count;

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

                    let chunk_size = range.len();
                    let mut buffer = vec![0u8; chunk_size];

                    file.read_at(&mut buffer, range.start as u64).unwrap();

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

pub fn process_chunk(data: Vec<u8>) -> RapidHashMap<Vec<u8>, Data> {
    let mut temps: RapidHashMap<Vec<u8>, Data> = RapidHashMap::with_capacity(500);

    let mut start = 0;
    let mut station_key: &[u8] = b"";

    loop {
        if let Some(semicolon_idx) = memchr(b';', &data[start..]) {
            station_key = &data[start..(start + semicolon_idx)];
            start += semicolon_idx + 1;
        }

        if let Some(newline_idx) = memchr(b'\n', &data[start..]) {
            let temperature = &data[start..(start + newline_idx)];
            let temp = process_temperature(temperature);

            if let Some(data) = temps.get_mut(station_key) {
                data.update(temp);
            } else {
                temps.insert(
                    station_key.to_vec(),
                    Data {
                        min: temp,
                        max: temp,
                        count: 1,
                        total: temp as i64,
                    },
                );
            }

            start += newline_idx + 1;
        } else {
            break; // no more newlines, end of chunk reached
        }
    }

    temps
}
// samply record --windows-symbol-server https://msdl.microsoft.com/download/symbols --breakpad-symbol-server https://symbols.mozilla.org/try/ --windows-symbol-server https://chromium-browser-symsrv.commondatastorage.googleapis.com target/release/brc
