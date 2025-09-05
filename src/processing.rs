use std::collections::HashMap;

use crate::fast_hash::FastHashBuilder;

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

pub fn process_chunk(data: &[u8]) -> HashMap<&[u8], Vec<i16>, FastHashBuilder> {
    let mut temps: HashMap<&[u8], Vec<i16>, FastHashBuilder> =
        HashMap::with_capacity_and_hasher(512, FastHashBuilder);

    let mut start = 0;
    let mut station_key: &[u8] = b"";

    for (i, &b) in data.iter().enumerate() {
        if b == b';' {
            station_key = &data[start..i];
            start = i + 1;
        } else if b == b'\n' {
            let temp = process_temperature(&data[start..i]);
            temps
                .entry(station_key)
                .and_modify(|temps| temps.push(temp))
                .or_insert_with(|| Vec::with_capacity(512));
            start = i + 1;
        }
    }

    temps
}
