use std::error::Error;

use common::{day::AdventDay, file::read_file};
use regex::Regex;

use crate::common_values::get_data_path;

pub struct Day3;
impl AdventDay for Day3 {
    fn puzzle1(&self, path: &str) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let binding = read_file(path)?;
        let memory_fragment = binding.as_str();
        let instructions = find_commands(memory_fragment);

        let mut result: i64 = 0;
        for instruction in instructions {
            result += instruction.first_number * instruction.second_number
        }

        Ok(result)
    }
    fn puzzle2(&self, path: &str) -> Result<impl std::fmt::Debug, Box<dyn Error>> {
        let binding = read_file(path)?;
        let memory_fragment = binding.as_str();
        let instructions = find_commands2(memory_fragment);

        let mut result: i64 = 0;
        for instruction in instructions {
            result += instruction.first_number * instruction.second_number
        }

        Ok(result)
    }

    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day3.txt";
        self.print_puzzles(3, day1_path.as_str(), day1_path.as_str())
    }
}

fn find_commands(string_fragment: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let matches: Vec<&str> = re.find_iter(string_fragment).map(|m| m.as_str()).collect();

    let mut instructions: Vec<Instruction> = Vec::new();
    for m in matches {
        // println!("match: {}", m);
        let number_string = m.get(3..).unwrap().replace("(", "").replace(")", "");
        let mut numbers = number_string.split(',');

        let first_number = numbers.next().unwrap().parse::<i64>().unwrap();
        let second_number = numbers.next().unwrap().parse::<i64>().unwrap();

        let instruction = Instruction {
            first_number,
            second_number,
        };
        instructions.push(instruction);
    }
    instructions
}

fn find_commands2(string_fragment: &str) -> Vec<Instruction> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
    let matches: Vec<&str> = re.find_iter(string_fragment).map(|m| m.as_str()).collect();

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut enable_instructions = true;
    for m in matches {
        // println!("match: {}", m);
        if m == "do()" {
            enable_instructions = true;
            continue;
        }
        if m == "don't()" {
            enable_instructions = false;
            continue;
        }
        if enable_instructions == false {
            continue;
        }
        let number_string = m.get(3..).unwrap().replace("(", "").replace(")", "");
        let mut numbers = number_string.split(',');

        let first_number = numbers.next().unwrap().parse::<i64>().unwrap();
        let second_number = numbers.next().unwrap().parse::<i64>().unwrap();

        let instruction = Instruction {
            first_number,
            second_number,
        };
        instructions.push(instruction);
    }
    instructions
}
struct Instruction {
    first_number: i64,
    second_number: i64,
}

enum Operator {
    Multiply,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = "data/day3-test.txt";
        let day = Day3;
        let result = day.puzzle1(path).unwrap();
        let expected: i64 = 161;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = "data/day3-test2.txt";
        let day = Day3;
        let result = day.puzzle2(path).unwrap();
        let expected: i64 = 48;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
