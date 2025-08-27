//!day_05.rs

use anyhow::Result;

#[derive(Clone, Copy)]
struct CategoryRange {
    start: u64,
    range: u64,
}

impl CategoryRange {
    fn single(value: u64) -> Self {
        CategoryRange {
            start: value,
            range: 1,
        }
    }
    fn new(start: u64, range: u64) -> Self {
        CategoryRange { start, range }
    }
    fn end(&self) -> u64 {
        self.start + self.range - 1
    }
}

struct TransferMap {
    source: CategoryRange,
    destination: CategoryRange,
}

impl From<&str> for TransferMap {
    fn from(value: &str) -> Self {
        let mut value_iter = value.split_ascii_whitespace();
        let destination_start = match value_iter.next() {
            Some(ds) => ds.parse::<u64>().expect("bad input"),
            None => panic!("bad input"),
        };
        let source_start = match value_iter.next() {
            Some(ss) => ss.parse::<u64>().expect("bad input"),
            None => panic!("bad input"),
        };
        let range = match value_iter.next() {
            Some(r) => r.parse::<u64>().expect("bad input"),
            None => panic!("bad input"),
        };
        if value_iter.next().is_some() {
            panic!("bad input");
        }
        Self {
            source: CategoryRange::new(source_start, range),
            destination: CategoryRange::new(destination_start, range),
        }
    }
}

impl TransferMap {
    fn transfer_category_range(
        &self,
        input_range: &CategoryRange,
    ) -> Option<(CategoryRange, Vec<CategoryRange>)> {
        let overlapping_start = input_range.start.max(self.source.start);
        let overlapping_end = input_range.end().min(self.source.end());
        if overlapping_start <= overlapping_end {
            let overlaping_range = CategoryRange::new(
                overlapping_start + self.destination.start - self.source.start,
                overlapping_end - overlapping_start + 1,
            );
            let mut remaining_ranges: Vec<CategoryRange> = Vec::new();
            if input_range.start < overlapping_start {
                let pre_range =
                    CategoryRange::new(input_range.start, overlapping_start - input_range.start);
                remaining_ranges.push(pre_range);
            }
            if input_range.end() > overlapping_end {
                let post_range =
                    CategoryRange::new(overlapping_end + 1, input_range.end() - overlapping_end);
                remaining_ranges.push(post_range);
            }
            return Some((overlaping_range, remaining_ranges));
        }
        None
    }
}

#[derive(Copy, Clone, Default)]
enum TransferMapType {
    #[default]
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl TransferMapType {
    fn next(&self) -> Option<Self> {
        let result = match self {
            TransferMapType::SeedToSoil => TransferMapType::SoilToFertilizer,
            TransferMapType::SoilToFertilizer => TransferMapType::FertilizerToWater,
            TransferMapType::FertilizerToWater => TransferMapType::WaterToLight,
            TransferMapType::WaterToLight => TransferMapType::LightToTemperature,
            TransferMapType::LightToTemperature => TransferMapType::TemperatureToHumidity,
            TransferMapType::TemperatureToHumidity => TransferMapType::HumidityToLocation,
            TransferMapType::HumidityToLocation => return None,
        };
        Some(result)
    }
}

#[derive(Default)]
struct TransferMapSet {
    seed_to_soil: Vec<TransferMap>,
    soil_to_fertilizer: Vec<TransferMap>,
    fertilizer_to_water: Vec<TransferMap>,
    water_to_light: Vec<TransferMap>,
    light_to_temperature: Vec<TransferMap>,
    temperature_to_humidity: Vec<TransferMap>,
    humidity_to_location: Vec<TransferMap>,
}

impl TransferMapSet {
    fn get_trans_map(&self, map_type: TransferMapType) -> &Vec<TransferMap> {
        match map_type {
            TransferMapType::SeedToSoil => &self.seed_to_soil,
            TransferMapType::SoilToFertilizer => &self.soil_to_fertilizer,
            TransferMapType::FertilizerToWater => &self.fertilizer_to_water,
            TransferMapType::WaterToLight => &self.water_to_light,
            TransferMapType::LightToTemperature => &self.light_to_temperature,
            TransferMapType::TemperatureToHumidity => &self.temperature_to_humidity,
            TransferMapType::HumidityToLocation => &self.humidity_to_location,
        }
    }
    fn get_trans_map_mut(&mut self, map_type: TransferMapType) -> &mut Vec<TransferMap> {
        match map_type {
            TransferMapType::SeedToSoil => &mut self.seed_to_soil,
            TransferMapType::SoilToFertilizer => &mut self.soil_to_fertilizer,
            TransferMapType::FertilizerToWater => &mut self.fertilizer_to_water,
            TransferMapType::WaterToLight => &mut self.water_to_light,
            TransferMapType::LightToTemperature => &mut self.light_to_temperature,
            TransferMapType::TemperatureToHumidity => &mut self.temperature_to_humidity,
            TransferMapType::HumidityToLocation => &mut self.humidity_to_location,
        }
    }
    fn add_transfer_map(&mut self, map: TransferMap, map_type: TransferMapType) {
        self.get_trans_map_mut(map_type).push(map);
    }

    fn get_min_location_from_seed_ranges(&self, mut seed_ranges: Vec<CategoryRange>) -> u64 {
        let mut map_type = Some(TransferMapType::default());
        while let Some(tmt) = map_type {
            let mut transfered_ranges: Vec<CategoryRange> = Vec::new();
            while let Some(input_range) = seed_ranges.pop() {
                let mut no_transfer = true;
                for transfer_map in self.get_trans_map(tmt).iter() {
                    if let Some((transfered_range, remaining_ranges)) =
                        transfer_map.transfer_category_range(&input_range)
                    {
                        transfered_ranges.push(transfered_range);
                        seed_ranges.extend_from_slice(&remaining_ranges[..]);
                        no_transfer = false;
                        break;
                    }
                }
                if no_transfer {
                    transfered_ranges.push(input_range);
                }
            }
            seed_ranges = transfered_ranges;
            map_type = tmt.next();
        }
        seed_ranges.iter().map(|cr| cr.start).min().unwrap()
    }
}

// The solution is an implementation in Rust of https://www.youtube.com/watch?v=NmxHw_bHhGM by HyperNeutrino
// www.youtube.com/@hyper-neutrino
// github.com/hyper-neutrino

pub fn day_05() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_05.txt");
    let mut transfer_maps = TransferMapSet::default();
    let mut seed_input: Vec<u64> = Vec::new();
    let mut transfer_map_type: Option<TransferMapType> = None;
    for line in input.lines().filter(|l| !l.is_empty()) {
        if seed_input.is_empty() {
            seed_input = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split_ascii_whitespace()
                .map(|u| u.parse::<u64>().expect("bad input"))
                .collect();
        } else if line.contains("map") {
            transfer_map_type = match transfer_map_type {
                Some(tmt) => tmt.next(),
                None => Some(TransferMapType::default()),
            };
        } else if let Some(tmt) = transfer_map_type {
            transfer_maps.add_transfer_map(TransferMap::from(line), tmt);
        }
    }

    let seeds: Vec<CategoryRange> = seed_input
        .iter()
        .map(|s| CategoryRange::single(*s))
        .collect();

    let lowest_location = transfer_maps.get_min_location_from_seed_ranges(seeds);
    println!("result day 05 part 1: {}", lowest_location);
    assert_eq!(lowest_location, 261_668_924);

    // Part 2
    let mut seeds: Vec<CategoryRange> = Vec::new();
    let mut seed_iter = seed_input.iter();
    while let Some(&start_seed) = seed_iter.next() {
        let &seed_range = seed_iter.next().expect("bad input");
        seeds.push(CategoryRange::new(start_seed, seed_range));
    }
    let lowest_location = transfer_maps.get_min_location_from_seed_ranges(seeds);
    println!("result day 05 part 2: {}", lowest_location);
    assert_eq!(lowest_location, 24_261_545);

    Ok(())
}
