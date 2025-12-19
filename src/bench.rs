extern crate test;

#[cfg(test)]
mod temperature_parsing_test {

    use super::*;
    use test::Bencher;

    #[bench]
    fn temperature_parsing(b: &mut Bencher) {
        b.iter(|| crate::processing::process_temperature("-38.1".as_bytes()));
    }
}

#[cfg(test)]
mod collect_temperatures {

    use super::*;
    use test::Bencher;

    #[bench]
    fn collect_temperatures(b: &mut Bencher) {
        b.iter(|| {
            crate::processing::process_chunk(
                "Weather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\nWeather-Stätion;-48.1\nStat;23.1\nStat;22.0\nWeather-Stätion;-28.1\n".as_bytes(),
            )
        });
    }
}
