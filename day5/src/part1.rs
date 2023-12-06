use std::collections::HashMap;
use std::ops::Range as StdRange;

use anyhow::{anyhow, Result};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum MapKey {
    SeedToSoil = 0,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map(Vec<Range>);

impl Map {
    pub fn new(map: Vec<Range>) -> Map {
        Self(map)
    }

    pub fn transpose(&self, src: usize) -> usize {
        self.0
            .iter()
            .find_map(|range| range.src_dst(src))
            .unwrap_or(src)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    dest: StdRange<usize>,
    src: StdRange<usize>,
}

impl From<(usize, usize, usize)> for Range {
    fn from(value: (usize, usize, usize)) -> Self {
        Self {
            dest: (value.0..value.0 + value.2),
            src: (value.1..value.1 + value.2),
        }
    }
}

impl Range {
    pub fn new(range: (usize, usize, usize)) -> Self {
        range.into()
    }

    pub fn contains_src(&self, source: usize) -> bool {
        self.src.contains(&source)
    }

    pub fn src_dst(&self, source: usize) -> Option<usize> {
        if !self.contains_src(source) {
            return None;
        }

        let offset = source - self.src.start;
        Some(self.dest.start + offset)
    }
}

pub fn process(input: &str) -> Result<usize> {
    let seeds = parse_seeds(input)?;
    println!("seeds: {:?}", seeds);
    let maps = parse_maps(input);

    let mut lowest = usize::MAX;

    seeds.iter().for_each(|seed| {
        let local = maps.iter().fold(*seed, |seed, map| map.transpose(seed));
        if local < lowest {
            lowest = local;
        }
    });

    Ok(lowest)
}

fn parse_seeds(input: &str) -> Result<Vec<usize>> {
    let mut lines = input.lines();
    let Some(line) = lines.next() else {
        return Err(anyhow!("Invalid input"));
    };

    let (_, numbers) = line
        .split_once(':')
        .ok_or(anyhow!("invalid seeds inputs"))?;

    let seeds = numbers
        .trim()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    Ok(seeds)
}

fn parse_maps(input: &str) -> Vec<Map> {
    let mut maps: HashMap<u8, Vec<Range>> = HashMap::new();

    let mut current_map: Option<u8> = None;
    let mut vec_map: Vec<Map> = Vec::with_capacity(MapKey::HumidityToLocation as usize + 1);

    for line in input.lines() {
        if line.ends_with("map:") {
            current_map = match line
                .replace(" map:", "")
                .replace('-', "_")
                .to_lowercase()
                .as_str()
            {
                "seed_to_soil" => Some(MapKey::SeedToSoil as u8),
                "soil_to_fertilizer" => Some(MapKey::SoilToFertilizer as u8),
                "fertilizer_to_water" => Some(MapKey::FertilizerToWater as u8),
                "water_to_light" => Some(MapKey::WaterToLight as u8),
                "light_to_temperature" => Some(MapKey::LightToTemperature as u8),
                "temperature_to_humidity" => Some(MapKey::TemperatureToHumidity as u8),
                "humidity_to_location" => Some(MapKey::HumidityToLocation as u8),
                _ => None,
            };
        } else if let Some(map_key) = &current_map {
            if !line.is_empty() && !line.starts_with("seeds:") {
                let values: Vec<usize> = line
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                maps.entry(*map_key)
                    .or_insert_with(Vec::new)
                    .push((values[0], values[1], values[2]).into());
            }
        }
    }

    for key in 0..=MapKey::HumidityToLocation as u8 {
        let m = Map::new(maps.get(&key).expect("map_empty").to_vec());
        vec_map.push(m);
    }

    vec_map
}
