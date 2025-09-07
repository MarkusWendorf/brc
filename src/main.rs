#![feature(test)]
mod bench;
mod data;
mod fast_hash;
mod processing;

use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::thread::{self, available_parallelism};

use crate::data::Data;
use crate::processing::process_chunk;

static FILE_PATH: &str = "data.txt";

fn main() -> io::Result<()> {
    let file = OpenOptions::new().read(true).open(FILE_PATH)?;
    let mut reader = BufReader::new(file);

    let total_length = reader.get_ref().metadata()?.len() as usize;

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
            reader.seek(SeekFrom::Start(chunk_end as u64))?;
            let mut buffer = Vec::new();

            chunk_end = match reader.read_until(b'\n', &mut buffer) {
                Ok(_) => reader.stream_position()? as usize,
                _ => total_length,
            };
        }

        indices.push(chunk_start..chunk_end);
        offset = chunk_end;
    }

    thread::scope(|scope| {
        let parts = indices
            .into_iter()
            .map(|range| {
                scope.spawn(move || {
                    let file = OpenOptions::new().read(true).open(FILE_PATH).unwrap();
                    let mut reader = BufReader::new(file);
                    reader.seek(SeekFrom::Start(range.start as u64)).unwrap();

                    let mut buffer = vec![0u8; range.len()];
                    reader.read_exact(&mut buffer).unwrap();

                    let data = process_chunk(&buffer);

                    let mut part: HashMap<String, Data> = HashMap::new();
                    for (station, temperatures) in data {
                        let mut data = Data::default();
                        data.station = String::from_utf8_lossy(station).into_owned();

                        for temperature in temperatures {
                            data.update(temperature);
                        }
                        part.insert(data.station.clone(), data);
                    }
                    part
                })
            })
            .collect::<Vec<_>>();

        let processed_parts: Vec<HashMap<String, Data>> =
            parts.into_iter().map(|p| p.join().unwrap()).collect();

        let mut combined: BTreeMap<String, Data> = BTreeMap::new();
        for part in processed_parts {
            for (key, data) in part {
                combined
                    .entry(key)
                    .and_modify(|d| d.merge(&data))
                    .or_insert(data);
            }
        }

        let mut stdout = std::io::stdout().lock();
        stdout.write_all(b"{").unwrap();

        let last_index = combined.len() - 1;
        for (i, (station, data)) in combined.iter().enumerate() {
            let mean = data.total as f32 / data.count as f32;
            let line = format!(
                "{}={}/{}/{}",
                station,
                format_number(data.min),
                format_mean(mean),
                format_number(data.max)
            );

            stdout.write_all(line.as_bytes()).unwrap();

            if i != last_index {
                stdout.write_all(b", ").unwrap();
            }
        }

        stdout.write_all(b"}").unwrap();
    });

    Ok(())
}

#[inline(always)]
fn format_number(value: i16) -> String {
    (value as f32 / 10.0).to_string()
}

#[inline(always)]
fn format_mean(value: f32) -> String {
    ((value).round() / 10.0).to_string()
}
