use core::num;
use std::{collections::HashMap, error::Error, fs, path::Path};

pub fn print_puzzles() {
    let path = Path::new("data/day5-input.txt");
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

#[derive(Clone)]
struct Mapping {
    /** second number */
    source: i32,
    /** first number */
    destination: i32,
    /** third number */
    range: i32,
}

fn puzzle1(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;

    let mut seeds: Vec<i32> = Vec::new();
    let mut sections: HashMap<&str, Section> = HashMap::new();
    let mut section: Option<Section> = None;
    for line in input_value.lines() {
        if line.starts_with("seeds:") {
            seeds = line
                .replace("seeds: ", "")
                .split(' ')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();
            continue;
        }
        if line.contains("map:") {
            let name = line.replace(" map:", "");
            section = Some(Section {
                name,
                mappings: Vec::new(),
            });
            continue;
        }
        if let Some(s) = &mut section {
            if !line.is_empty() {
                let numbers: Vec<i32> = line
                    .trim()
                    .split(' ')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                let mapping = Mapping {
                    destination: numbers[0],
                    source: numbers[1],
                    range: numbers[3],
                };
                s.mappings.push(mapping);
            } else {
                sections.insert(&s.name.as_str(), s.clone());
            }
        }
    }
    todo!()
}

fn puzzle2(path: &Path) -> Result<i32, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_DATA: &str = "data/day5-test-input.txt";
    #[test]
    fn test_puzzle1() {
        let result = puzzle1(Path::new(TEST_DATA));

        assert_eq!(13, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(Path::new(TEST_DATA));

        assert_eq!(30, result.unwrap())
    }
}
