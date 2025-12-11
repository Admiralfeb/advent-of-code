use std::{
    fs::{self},
    path::Path,
    sync::OnceLock,
    time::Instant,
};

use common::impl_day;
use regex::Regex;

use crate::common_values::YEAR;

pub struct Day;

static DIGIT_REGEX: OnceLock<Regex> = OnceLock::new();
static WORD_DIGIT_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_digit_regex() -> &'static Regex {
    DIGIT_REGEX.get_or_init(|| Regex::new(r"[0-9]").unwrap())
}

fn get_word_digit_regex() -> &'static Regex {
    WORD_DIGIT_REGEX.get_or_init(|| {
        Regex::new(r"^([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap()
    })
}

fn digit_word_to_char(s: &str) -> char {
    match s {
        "0" | "zero" => '0',
        "1" | "one" => '1',
        "2" | "two" => '2',
        "3" | "three" => '3',
        "4" | "four" => '4',
        "5" | "five" => '5',
        "6" | "six" => '6',
        "7" | "seven" => '7',
        "8" | "eight" => '8',
        "9" | "nine" => '9',
        _ => panic!("Invalid digit word: {}", s),
    }
}

impl_day!(1, YEAR, "day01-input.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let start = Instant::now();
        let input_value = fs::read_to_string(path)?;
        let regex = get_digit_regex();
        let mut result: i32 = 0;
        for line in input_value.lines() {
            let mut digits = regex.find_iter(line).filter_map(|m| m.as_str().chars().next());
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            let number = (first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()) as i32;
            result += number;
        }
        println!("Day 1 Puzzle 1 took: {:?}", start.elapsed());
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let start = Instant::now();
        let input_value = fs::read_to_string(path)?;
        let regex = get_word_digit_regex();
        let mut result: i32 = 0;
        for line in input_value.lines() {
            let digits: Vec<char> = line
                .char_indices()
                .filter_map(|(i, _)| regex.captures(&line[i..]))
                .map(|s| digit_word_to_char(&s[0]))
                .collect();

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            let number = (first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()) as i32;
            result += number;
        }
        println!("Day 1 Puzzle 2 took: {:?}", start.elapsed());
        Ok(result)
    },
});

/*
* Iterate through the digits in each line and create a number. Sum the results of all line numbers.
*/

#[cfg(test)]
mod test {

    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, "day01-test-input.txt");
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i32 = 142;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, "day01-test-input2.txt");
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i32 = 281;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }
}
