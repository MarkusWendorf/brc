use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
    io::Write,
};

use crate::{data::Data, fast_hash::FastHashBuilder};

pub fn process_chunk(data: &[u8]) -> HashMap<&[u8], Data, FastHashBuilder> {
    let mut temps: HashMap<&[u8], Data, FastHashBuilder> =
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
                .entry(station_key)
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
/// Processes byte slice and returns temperature as integer (multiplied by 10)
///
/// 25.5 -> 255
pub fn process_temperature(data: &[u8]) -> i16 {
    let is_negative = (data[0] == b'-') as i16;
    let first_number_index = is_negative as usize;
    let len = data.len();

    let first_digit = (data[first_number_index] - b'0') as i16;
    let second_digit = (data[first_number_index + 1] - b'0') as i16;

    let decimal_place = (data[len - 1] - b'0') as i16;

    // if we have 4 characters from the start of the first number we have two digits (example: 22.0)
    let is_double_digit = (len - first_number_index == 4) as i16;

    // 2 digits = first_digit * 100 + second_digit * 10
    // 1 digit  = first_digit * 10  + second_digit * 0   (second_digit is '.' in this case which should not be added)
    let integer_part =
        first_digit * (10 + 90 * is_double_digit) + second_digit * (10 * is_double_digit);

    let sum = integer_part + decimal_place;
    let sign = 1 - 2 * is_negative;

    sign * sum
}

#[inline(always)]
pub fn process_temperature_simple(data: &[u8]) -> i16 {
    let mut sum: i16 = 0;
    let mut exponent = 0;

    for val in data.iter().rev() {
        match val {
            b'.' => continue,
            b'-' => sum *= -1,
            digit => {
                sum += (digit - b'0') as i16 * 10i16.pow(exponent);
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
