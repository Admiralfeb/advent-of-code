use std::{error::Error, fs, path::Path};

use regex::Regex;

pub fn print_puzzles() {
    let path = Path::new("data/day3-input.txt");
    println!(
        "day 3 results: {}, {}",
        puzzle1(path).unwrap(),
        puzzle2(path).unwrap()
    )
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: i32,
    line: usize,
    location: usize,
    length: usize,
    is_part_number: bool,
}

#[derive(Debug)]
struct Symbol {
    line: usize,
    location: usize,
}

/** Get Part numbers and sum

 Part numbers will be directly adjacent to symbols, `.` not included.
*/
fn puzzle1(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (i, line) in input_value.lines().enumerate() {
        let number_regex = Regex::new(r"[0-9]+").unwrap();
        let mut line_numbers: Vec<Number> = number_regex
            .find_iter(line)
            .map(|m| Number {
                value: m.as_str().parse().unwrap(),
                line: i,
                location: m.start(),
                length: m.len(),
                is_part_number: false,
            })
            .collect();
        let symbol_regex = Regex::new(r"[\W&&[^\.\s]]").unwrap();
        let mut line_symbols: Vec<Symbol> = symbol_regex
            .find_iter(line)
            .map(|m| Symbol {
                line: i,
                location: m.start(),
            })
            .collect();
        numbers.append(&mut line_numbers);
        symbols.append(&mut line_symbols);
    }

    for Symbol { line, location } in symbols {
        numbers = numbers
            .into_iter()
            .map(|n| {
                if n.line == line - 1 || n.line == line || n.line == line + 1 {
                    // check location, including length
                    // if len is 3 and location is 0, then 0-3 is the range that is acceptable
                    let start_location = match n.location {
                        0 => 0,
                        _ => n.location - 1,
                    };
                    for i in start_location..=n.location + n.length {
                        if location == i {
                            return Number {
                                is_part_number: true,
                                ..n
                            };
                        }
                    }
                };
                n
            })
            .collect();
    }

    let mut result = 0;
    for part_number in numbers.iter().filter(|n| n.is_part_number) {
        result += part_number.value;
    }

    Ok(result)
}

fn puzzle2(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;

    let mut numbers: Vec<Number> = Vec::new();
    let mut possible_gears: Vec<Symbol> = Vec::new();

    for (i, line) in input_value.lines().enumerate() {
        let number_regex = Regex::new(r"[0-9]+").unwrap();
        let mut line_numbers: Vec<Number> = number_regex
            .find_iter(line)
            .map(|m| Number {
                value: m.as_str().parse().unwrap(),
                line: i,
                location: m.start(),
                length: m.len(),
                is_part_number: false,
            })
            .collect();
        let symbol_regex = Regex::new(r"\*").unwrap();
        let mut line_possible_gears: Vec<Symbol> = symbol_regex
            .find_iter(line)
            .map(|m| Symbol {
                line: i,
                location: m.start(),
            })
            .collect();
        numbers.append(&mut line_numbers);
        possible_gears.append(&mut line_possible_gears);
    }

    let mut result = 0;
    for Symbol { line, location } in possible_gears {
        let geared_numbers: Vec<i32> = numbers
            .iter()
            .filter(|n| {
                if n.line == line - 1 || n.line == line || n.line == line + 1 {
                    // check location, including length
                    // if len is 3 and location is 0, then 0-3 is the range that is acceptable
                    let start_location = match n.location {
                        0 => 0,
                        _ => n.location - 1,
                    };
                    for i in start_location..=n.location + n.length {
                        if location == i {
                            return true;
                        }
                    }
                };
                false
            })
            .map(|v| v.value)
            .collect();

        if geared_numbers.len() == 2 {
            result += geared_numbers[0] * geared_numbers[1];
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(Path::new("data/day3-test-input.txt"));

        assert_eq!(4361, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(Path::new("data/day3-test-input.txt"));

        assert_eq!(467835, result.unwrap())
    }
}
