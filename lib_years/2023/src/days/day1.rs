use std::{
    error::Error,
    fmt::Debug,
    fs::{self},
};

use common::day::AdventDay;
use regex::Regex;

use crate::common_values::get_data_path;

pub struct Day1;
impl AdventDay for Day1 {
    fn puzzle1(&self, path: &str) -> Result<impl Debug, Box<dyn Error>> {
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
    }

    fn puzzle2(&self, path: &str) -> Result<impl Debug, Box<dyn Error>> {
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
    }
    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day1-input.txt";
        self.print_puzzles(1, day1_path.as_str(), day1_path.as_str())
    }
}

/*
* Iterate through the digits in each line and create a number. Sum the results of all line numbers.
*/

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = "data/day1-test-input.txt";
        let day = Day1;
        let result = day.puzzle1(path).unwrap();
        let expected: i32 = 142;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }

    #[test]
    fn test_puzzle2() {
        let path = "data/day1-test-input2.txt";
        let day = Day1;
        let result = day.puzzle2(path).unwrap();
        let expected: i32 = 281;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }
}
