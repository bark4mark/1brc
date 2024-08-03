use std::{collections::BTreeMap, fs};

#[derive(Debug)]
struct Weather {
    min: f32,
    max: f32,
    avg: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let path = match std::env::args().skip(1).next() {
        Some(path) => path,
        None => "measurements.txt".to_owned(),
    };
    let contents = fs::read_to_string(path)?;
    let mut weather_map: BTreeMap<String, Weather> = BTreeMap::new();
    for line in contents.lines() {
        let mut iter = line.split(";");
        let city = iter.next().unwrap();
        let temp: f32 = iter.next().unwrap().parse().unwrap();
        let entry = weather_map.entry(city.to_string()).or_insert(Weather { min: temp, max: temp, avg: temp });
        if temp < entry.min {
            entry.min = temp;
        }
        if temp > entry.max {
            entry.max = temp;
        }
    }
    for (_city, mut weather) in weather_map {
        weather.avg = (weather.min + weather.max) / 2.0;
        println!("{};{};{};{}", _city, weather.min, weather.max, weather.avg);
    }

    Ok(())
}