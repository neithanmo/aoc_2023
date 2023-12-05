use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum MapKey {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map(HashMap<MapKey, Vec<Range>>);

impl Map {
    pub fn new(map: HashMap<MapKey, Vec<Range>>) -> Map {
        Self(map)
    }

    pub fn seed_to_soil(&self) -> &[Range] {
        self.0.get(&MapKey::SeedToSoil).expect("Map empty!!")
    }

    pub fn soil_to_fertilizer(&self) -> &[Range] {
        self.0.get(&MapKey::SoilToFertilizer).expect("Map empty!!")
    }

    pub fn fertilizer_to_water(&self) -> &[Range] {
        self.0.get(&MapKey::FertilizerToWater).expect("Map empty!!")
    }
    pub fn water_to_light(&self) -> &[Range] {
        self.0.get(&MapKey::WaterToLight).expect("Map empty!!")
    }
    pub fn light_to_temperature(&self) -> &[Range] {
        self.0
            .get(&MapKey::LightToTemperature)
            .expect("Map empty!!")
    }
    pub fn temperature_to_humidity(&self) -> &[Range] {
        self.0
            .get(&MapKey::TemperatureToHumidity)
            .expect("Map empty!!")
    }

    pub fn humidity_to_location(&self) -> &[Range] {
        self.0
            .get(&MapKey::HumidityToLocation)
            .expect("Map empty!!")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range(usize, usize, usize);
impl From<(usize, usize, usize)> for Range {
    fn from(value: (usize, usize, usize)) -> Self {
        Range::new(value.0, value.1, value.2)
    }
}

impl Range {
    pub fn new(destination: usize, source: usize, range: usize) -> Self {
        Self(destination, source, range)
    }

    pub fn src_range(&self) -> std::ops::Range<usize> {
        self.1..self.1 + self.2
    }

    pub fn dst_range(&self) -> std::ops::Range<usize> {
        self.0..self.0 + self.2
    }

    pub fn range(&self) -> usize {
        self.2
    }

    pub fn contains_src(&self, source: usize) -> bool {
        self.src_range().contains(&source)
    }

    pub fn src_dst(&self, source: usize) -> Option<usize> {
        if !self.contains_src(source) {
            return None;
        }

        let offset = source - self.1;
        Some(self.0 + offset)
    }
}

pub fn process(input: &str) -> Result<usize> {
    let seeds = parse_seeds(input)?;
    println!("seeds: {:?}", seeds);
    let map = parse_maps(input);
    println!("{:?}", map);

    let mut lowest = usize::MAX;

    for seed in seeds.iter() {
        // seed -> soil
        let range = map.seed_to_soil();
        let mut source = range.iter().find_map(|r| r.src_dst(*seed)).unwrap_or(*seed);
        // soil -> fertilizer
        let range = map.soil_to_fertilizer();
        source = range
            .iter()
            .find_map(|r| r.src_dst(source))
            .unwrap_or(source);
        // fertilizer -> water
        let range = map.fertilizer_to_water();
        source = range
            .iter()
            .find_map(|r| r.src_dst(source))
            .unwrap_or(source);
        // water -> light
        let range = map.water_to_light();
        source = range
            .iter()
            .find_map(|r| r.src_dst(source))
            .unwrap_or(source);
        // light -> temperature
        let range = map.light_to_temperature();
        source = range
            .iter()
            .find_map(|r| r.src_dst(source))
            .unwrap_or(source);
        // temperature -> humidity
        let range = map.temperature_to_humidity();
        source = range
            .iter()
            .find_map(|r| r.src_dst(source))
            .unwrap_or(source);
        // humidity -> location
        let range = map.humidity_to_location();
        source = range
            .iter()
            .find_map(|r| r.src_dst(source))
            .unwrap_or(source);

        if source < lowest {
            lowest = source;
        }
    }

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

fn parse_maps(input: &str) -> Map {
    let mut maps: HashMap<MapKey, Vec<Range>> = HashMap::new();
    let mut current_map: Option<MapKey> = None;

    for line in input.lines() {
        if line.ends_with("map:") {
            current_map = match line
                .replace(" map:", "")
                .replace("-", "_")
                .to_lowercase()
                .as_str()
            {
                "seed_to_soil" => Some(MapKey::SeedToSoil),
                "soil_to_fertilizer" => Some(MapKey::SoilToFertilizer),
                "fertilizer_to_water" => Some(MapKey::FertilizerToWater),
                "water_to_light" => Some(MapKey::WaterToLight),
                "light_to_temperature" => Some(MapKey::LightToTemperature),
                "temperature_to_humidity" => Some(MapKey::TemperatureToHumidity),
                "humidity_to_location" => Some(MapKey::HumidityToLocation),
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

    Map::new(maps)
}
