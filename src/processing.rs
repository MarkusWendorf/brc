use std::collections::HashMap;

use crate::{data::Data, fast_hash::FastHashBuilder};

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

pub fn process_chunk(data: Vec<u8>) -> HashMap<Vec<u8>, Data, FastHashBuilder> {
    let mut temps: HashMap<Vec<u8>, Data, FastHashBuilder> =
        HashMap::with_capacity_and_hasher(512, FastHashBuilder);

    let mut start = 0;
    let mut station_key: &[u8] = b"";

    for (i, &b) in data.iter().enumerate() {
        if b == b';' {
            // get_unchecked is safe because we know the range exists because we just iterated over it
            station_key = unsafe { data.get_unchecked(start..i) };
            start = i + 1;
        } else if b == b'\n' {
            // get_unchecked is safe because we know the range exists because we just iterated over it
            let temperature = unsafe { data.get_unchecked(start..i) };

            let temp = process_temperature(temperature);

            let station = Vec::from(station_key);

            temps
                .entry(station.clone())
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
