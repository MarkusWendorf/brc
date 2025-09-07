mod buffered_reader;
mod data;
mod fast_hash;
mod memory_mapped;
mod processing;

use std::collections::{BTreeMap};
use std::io::{self, Write};

use crate::buffered_reader::buffered_reader;
use crate::data::Data;
use crate::memory_mapped::memory_mapped;

static FILE_PATH: &str = "data.txt";

fn main() -> io::Result<()> {
    let processed_parts = memory_mapped(FILE_PATH);

    let mut combined: BTreeMap<Vec<u8>, Data> = BTreeMap::new();
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
            String::from_utf8_lossy(station),
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
