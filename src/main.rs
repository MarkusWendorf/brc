#![feature(test)]
mod bench;
mod data;
mod fast_hash;
mod processing;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::{self, Write};

use std::thread;

use memmap2::MmapMut;

use crate::data::Data;
use crate::processing::process_chunk;

fn main() -> io::Result<()> {
    let file = OpenOptions::new().read(true).write(true).open("data.txt")?;

    let mut mmap = unsafe { MmapMut::map_mut(&file)? };
    mmap.advise(memmap2::Advice::Sequential).unwrap();

    let chunk_count = 10;
    let mut indices: [std::ops::Range<usize>; 10] = Default::default();

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
            .map(|chunk| {
                scope.spawn(|| {
                    let data = process_chunk(chunk);

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
            .into_iter();

        let processed_parts: Vec<HashMap<String, Data>> =
            parts.map(|p| p.join().unwrap()).collect();

        let mut station_keys: HashSet<&String> = HashSet::new();

        for part in processed_parts.iter() {
            for key in part.keys() {
                station_keys.insert(key);
            }
        }

        let mut combined: BTreeMap<String, Data> = BTreeMap::new();

        for part in processed_parts.iter() {
            for j in station_keys.iter() {
                let key = *j;
                if let Some(data) = part.get(key) {
                    combined
                        .entry(key.clone())
                        .and_modify(|d| d.merge(data))
                        .or_insert(data.clone());
                }
            }
        }

        {
            let mut stdout = std::io::stdout().lock();
            stdout.write(b"{").unwrap();

            let last_index = combined.len() - 1;
            for (i, (station, data)) in combined.iter().enumerate() {
                let mean = data.total as f32 / data.count as f32;

                let line = station.to_owned()
                    + "="
                    + format_number(data.min).as_str()
                    + "/"
                    + format_mean(mean).as_str()
                    + "/"
                    + format_number(data.max).as_str();

                stdout.write(&line.as_bytes()).unwrap();

                if i != last_index {
                    stdout.write(b", ").unwrap();
                }
            }

            stdout.write(b"}").unwrap();
        }
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
