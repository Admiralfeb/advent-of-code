use std::{error::Error, fs, path::Path, time::Instant};

use common::impl_day;

use crate::common_values::YEAR;

pub struct Day;

#[derive(Clone)]
struct Section {
    name: String,
    mappings: Vec<Mapping>,
}

impl Section {
    fn find_mapping(&self, source_value: &i64) -> i64 {
        let mapping = self
            .mappings
            .iter()
            .find_map(|m| m.find_value(source_value));
        match mapping {
            Some(x) => x,
            None => *source_value,
        }
    }
}

struct Seed {
    seed_number: i64,
    seed_location: i64,
}

#[derive(Clone)]
struct Mapping {
    /** second number */
    source: i64,
    /** first number */
    destination: i64,
    /** third number */
    range: i64,
}

impl Mapping {
    fn find_value(&self, source_value: &i64) -> Option<i64> {
        let bounds = (self.source, self.source + self.range);
        if bounds.0 <= *source_value && *source_value < bounds.1 {
            let offset = source_value - bounds.0;
            Some(self.destination + offset)
        } else {
            None
        }
        // let sources: Vec<i64> = (self.source..=self.source + self.range).collect();
        // let destinations: Vec<i64> = (self.destination..=self.destination + self.range).collect();
        // let position = sources.iter().position(|s| s == source_value);
        // position.map(|x| destinations[x])
    }
}

impl_day!(5, YEAR, "day05-input.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let start = Instant::now();
        let result = Day::puzzle1(path)?;
        println!("Day 5 Puzzle 1 took: {:?}", start.elapsed());
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let start = Instant::now();
        let result = Day::puzzle2(path)?;
        println!("Day 5 Puzzle 2 took: {:?}", start.elapsed());
        Ok(result)
    },
});

impl Day {
    fn puzzle1(path: &Path) -> Result<i64, Box<dyn Error>> {
        let input_value = fs::read_to_string(path)?;

        let mut seeds: Vec<i64> = Vec::new();
        let input_sections = input_value.split("\n\n");
        let mut sections: Vec<Section> = Vec::new();

        for input_section in input_sections {
            if input_section.starts_with("seeds:") {
                seeds = input_section
                    .replace("seeds: ", "")
                    .split(' ')
                    .map(|s| s.trim().parse::<i64>().unwrap())
                    .collect();
                continue;
            }

            let mut section_name: String = String::from("");
            let mut mappings: Vec<Mapping> = Vec::new();
            for section_line in input_section.lines() {
                if section_line.contains("map:") {
                    section_name = section_line.replace(" map:", "");
                    continue;
                } else {
                    let numbers: Vec<i64> = section_line
                        .trim()
                        .split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect();
                    let mapping = Mapping {
                        destination: numbers[0],
                        source: numbers[1],
                        range: numbers[2],
                    };
                    mappings.push(mapping);
                }
            }
            let new_section = Section {
                name: section_name,
                mappings,
            };
            sections.push(new_section);
        }

        let closest_seed = seeds
            .iter()
            .map(|seed| {
                // find soil
                let soil = Day::process_section(&sections, "seed-to-soil", seed);

                // find fertilizer
                let fertilizer = Day::process_section(&sections, "soil-to-fertilizer", &soil);

                // find water
                let water = Day::process_section(&sections, "fertilizer-to-water", &fertilizer);

                // find light
                let light = Day::process_section(&sections, "water-to-light", &water);

                // find temp
                let temp = Day::process_section(&sections, "light-to-temperature", &light);

                // find humidity
                let humidity = Day::process_section(&sections, "temperature-to-humidity", &temp);

                // find location
                let location = Day::process_section(&sections, "humidity-to-location", &humidity);

                Seed {
                    seed_number: *seed,
                    seed_location: location,
                }
            })
            .min_by_key(|seed| seed.seed_location)
            .unwrap();

        println!(
            "closest seed: {}, location: {}",
            closest_seed.seed_number, closest_seed.seed_location
        );
        Ok(closest_seed.seed_location)
    }

    fn process_section(sections: &[Section], section_name: &str, source_value: &i64) -> i64 {
        let section = sections.iter().find(|s| s.name == section_name).unwrap();

        section.find_mapping(source_value)
    }

    fn puzzle2(path: &Path) -> Result<i64, Box<dyn Error>> {
        let input_value = fs::read_to_string(path)?;

        let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
        let input_sections = input_value.split("\n\n");
        let mut sections: Vec<Section> = Vec::new();

        for input_section in input_sections {
            if input_section.starts_with("seeds:") {
                let input_seeds: Vec<i64> = input_section
                    .replace("seeds: ", "")
                    .split(' ')
                    .map(|s| s.trim().parse::<i64>().unwrap())
                    .collect();
                for i in (0..input_seeds.len()).step_by(2) {
                    seed_ranges.push((input_seeds[i], input_seeds[i] + input_seeds[i + 1]));
                }
                continue;
            }

            let mut section_name: String = String::from("");
            let mut mappings: Vec<Mapping> = Vec::new();
            for section_line in input_section.lines() {
                if section_line.contains("map:") {
                    section_name = section_line.replace(" map:", "");
                    continue;
                } else {
                    let numbers: Vec<i64> = section_line
                        .trim()
                        .split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect();
                    let mapping = Mapping {
                        destination: numbers[0],
                        source: numbers[1],
                        range: numbers[2],
                    };
                    mappings.push(mapping);
                }
            }
            let new_section = Section {
                name: section_name,
                mappings,
            };
            sections.push(new_section);
        }

        // Transform ranges through all sections
        let section_order = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];

        let mut current_ranges = seed_ranges;
        for section_name in &section_order {
            let section = sections.iter().find(|s| s.name == *section_name).unwrap();
            current_ranges = transform_ranges(current_ranges, &section.mappings);
        }

        // Find minimum start value across all final ranges
        let min_location = current_ranges
            .iter()
            .map(|(start, _)| *start)
            .min()
            .unwrap();

        Ok(min_location)
    }
}

// Transform a list of ranges through a set of mappings
fn transform_ranges(ranges: Vec<(i64, i64)>, mappings: &[Mapping]) -> Vec<(i64, i64)> {
    let mut result = Vec::new();

    for (range_start, range_end) in ranges {
        let mut to_process = vec![(range_start, range_end)];
        let mut processed = Vec::new();

        // For each mapping, try to transform parts of the range
        for mapping in mappings {
            let map_start = mapping.source;
            let map_end = mapping.source + mapping.range;
            let offset = mapping.destination - mapping.source;

            let mut unprocessed = Vec::new();

            for (start, end) in to_process {
                // Check for overlap with this mapping
                if end <= map_start || start >= map_end {
                    // No overlap, keep for next mapping
                    unprocessed.push((start, end));
                } else {
                    // There's overlap, split the range
                    // Part before mapping
                    if start < map_start {
                        unprocessed.push((start, map_start));
                    }
                    // Part within mapping (transform it)
                    let overlap_start = start.max(map_start);
                    let overlap_end = end.min(map_end);
                    processed.push((overlap_start + offset, overlap_end + offset));
                    // Part after mapping
                    if end > map_end {
                        unprocessed.push((map_end, end));
                    }
                }
            }

            to_process = unprocessed;
        }

        // Add any remaining unprocessed ranges (they pass through unchanged)
        result.extend(processed);
        result.extend(to_process);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use common::file::get_data_path;

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, "day05-test-input.txt");
        let result = Day::puzzle1(&path);

        assert_eq!(35, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, "day05-test-input.txt");
        let result = Day::puzzle2(&path);

        assert_eq!(46, result.unwrap())
    }
}
