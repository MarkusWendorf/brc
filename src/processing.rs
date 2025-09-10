use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
    io::Write,
};

use crate::{data::Data, fast_hash::FastHashBuilder};

pub fn process_chunk<'a, K, F>(data: &'a [u8], map_key_fn: F) -> HashMap<K, Data, FastHashBuilder>
where
    K: Eq + Hash,
    F: Fn(&'a [u8]) -> K,
{
    let mut temps: HashMap<K, Data, FastHashBuilder> =
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
                .entry(map_key_fn(station_key))
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

#[inline(always)]
pub fn process_temperature(data: &[u8]) -> i16 {
    let mut sum: i16 = 0;
    let mut exponent = 0;

    for val in data.iter().rev() {
        match val {
            b'.' => continue,
            b'-' => sum *= -1,
            digit => {
                sum += (digit - 48) as i16 * 10i16.pow(exponent);
                exponent += 1;
            }
        }
    }

    sum
}

pub fn output_results<K>(chunks: Vec<HashMap<K, Data, FastHashBuilder>>)
where
    K: Borrow<[u8]> + Eq + Hash + Ord,
{
    let mut combined: BTreeMap<K, Data> = BTreeMap::new();

    for part in chunks {
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
            String::from_utf8_lossy(station.borrow()), // works for both Vec<u8> and &[u8]
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
}

#[inline(always)]
fn format_number(value: i16) -> String {
    (value as f32 / 10.0).to_string()
}

#[inline(always)]
fn format_mean(value: f32) -> String {
    ((value).round() / 10.0).to_string()
}
