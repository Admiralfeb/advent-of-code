use std::{
    error::Error,
    fs,
    path::Path,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub fn print_puzzles() {
    let path = Path::new("data/day05-input.txt");
    println!(
        "day 5 results: {}, {}",
        puzzle1(path).unwrap(),
        puzzle2(path).unwrap()
    )
}

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
            let soil = process_section(&sections, "seed-to-soil", seed);

            // find fertilizer
            let fertilizer = process_section(&sections, "soil-to-fertilizer", &soil);

            // find water
            let water = process_section(&sections, "fertilizer-to-water", &fertilizer);

            // find light
            let light = process_section(&sections, "water-to-light", &water);

            // find temp
            let temp = process_section(&sections, "light-to-temperature", &light);

            // find humidity
            let humidity = process_section(&sections, "temperature-to-humidity", &temp);

            // find location
            let location = process_section(&sections, "humidity-to-location", &humidity);

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

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let input_sections = input_value.split("\n\n");
    let mut mut_sections: Vec<Section> = Vec::new();

    for input_section in input_sections {
        if input_section.starts_with("seeds:") {
            let input_seeds: Vec<i64> = input_section
                .replace("seeds: ", "")
                .split(' ')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            for i in 0..input_seeds.len() {
                if i % 2 != 0 {
                    continue;
                }
                let seed: (i64, i64) = (input_seeds[i], input_seeds[i + 1]);
                seeds.push(seed);
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
        mut_sections.push(new_section);
    }

    let arc_sections = Arc::new(mut_sections);

    let handles: Vec<JoinHandle<Seed>> = seeds
        .into_iter()
        .map(|seed| {
            let sections = Arc::clone(&arc_sections);
            thread::spawn(move || {
                println!("thread started for seed: {}", seed.0);
                let s = (seed.0..seed.0 + seed.1)
                    .map(|s| {
                        // find soil
                        let soil = process_section(&sections, "seed-to-soil", &s);

                        // find fertilizer
                        let fertilizer = process_section(&sections, "soil-to-fertilizer", &soil);

                        // find water
                        let water = process_section(&sections, "fertilizer-to-water", &fertilizer);

                        // find light
                        let light = process_section(&sections, "water-to-light", &water);

                        // find temp
                        let temp = process_section(&sections, "light-to-temperature", &light);

                        // find humidity
                        let humidity = process_section(&sections, "temperature-to-humidity", &temp);

                        // find location
                        let location =
                            process_section(&sections, "humidity-to-location", &humidity);

                        Seed {
                            seed_number: s,
                            seed_location: location,
                        }
                    })
                    .min_by_key(|s| s.seed_location)
                    .unwrap();

                println!("seed: {}, location: {}", s.seed_number, s.seed_location);
                s
            })
        })
        .collect();

    let closest = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .min_by_key(|s| s.seed_location)
        .unwrap();

    println!(
        "closest seed: {}, location: {}",
        closest.seed_number, closest.seed_location
    );
    Ok(closest.seed_location)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_DATA: &str = "data/day05-test-input.txt";
    #[test]
    fn test_puzzle1() {
        let result = puzzle1(Path::new(TEST_DATA));

        assert_eq!(35, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(Path::new(TEST_DATA));

        assert_eq!(46, result.unwrap())
    }
}
