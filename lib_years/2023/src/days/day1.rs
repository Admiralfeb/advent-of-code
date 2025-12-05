use std::{
    fs::{self},
    path::Path,
};

use common::impl_day;
use regex::Regex;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(1, YEAR, "day1-input.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let input_value = fs::read_to_string(path)?;
        let mut result: i32 = 0;
        for line in input_value.lines() {
            let regex = Regex::new(r"[0-9]").unwrap();
            let digits: Vec<String> = regex
                .find_iter(line)
                .filter_map(|digit| digit.as_str().parse::<i32>().ok())
                .map(|digit| digit.to_string())
                .collect();
            let r = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            let number = r.parse::<i32>().ok().unwrap();
            result += number;
        }
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let input_value = fs::read_to_string(path)?;
        let mut result: i32 = 0;
        for line in input_value.lines() {
            let regex =
                Regex::new(r"^([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let digs: Vec<String> = line
                .char_indices()
                .filter_map(|(i, _)| regex.captures(&line[i..]))
                .filter_map(|s| match s[0].to_string().as_str() {
                    "0" | "zero" => Some("0"),
                    "1" | "one" => Some("1"),
                    "2" | "two" => Some("2"),
                    "3" | "three" => Some("3"),
                    "4" | "four" => Some("4"),
                    "5" | "five" => Some("5"),
                    "6" | "six" => Some("6"),
                    "7" | "seven" => Some("7"),
                    "8" | "eight" => Some("8"),
                    "9" | "nine" => Some("9"),
                    _ => None,
                })
                .map(|v| v.to_string())
                .collect();

            let r = format!("{}{}", digs.first().unwrap(), digs.last().unwrap());
            let number = r.parse::<i32>().ok().unwrap();
            result += number;
        }
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
        let path = get_data_path(YEAR, "day1-test-input.txt");
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i32 = 142;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, "day1-test-input2.txt");
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i32 = 281;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }
}
