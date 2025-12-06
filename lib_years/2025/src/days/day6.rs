use common::{file::process_file_into_lines, impl_day};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use regex::Regex;

use std::{path::Path, sync::LazyLock};

use crate::common_values::YEAR;

static NUMERIC_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
static OP_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\+|\*").unwrap());

pub struct Day;

impl_day!(6, YEAR, "day6.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let mut number_sets: Vec<Vec<i64>> = vec![];
        let mut operations: Vec<Operation> = vec![];
        for line in lines {
            let values: Vec<i64> = NUMERIC_REGEX
                .find_iter(&line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            if values.len() > 0 {
                number_sets.push(values);
            }

            operations = OP_REGEX
                .find_iter(&line)
                .map(|m| Operation::from(m.as_str()))
                .collect();
        }

        let result: i64 = operations
            .into_par_iter()
            .enumerate()
            .map(|(i, operation)| -> i64 {
                let values: Vec<i64> = number_sets.iter().map(|ns| ns[i]).collect();
                match operation {
                    Operation::Add => values.iter().sum(),
                    Operation::Multiply => values.iter().product(),
                }
            })
            .sum();

        Ok(result)
    },

    puzzle2: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let longest_line = lines
            .iter()
            .enumerate()
            .max_by_key(|(_, l)| l.len())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let mut numbers: Vec<i64> = vec![];
        let mut problems: Vec<Problem> = vec![];
        for (col, _) in lines[longest_line].char_indices().rev() {
            let values: Vec<char> = lines.iter()
                .map(|l| {
                    l.chars().nth(col).unwrap_or(' ')
                })
                .collect();

            if values.iter().all(|v| v.is_whitespace()) {
                continue;
            }
            let number_val: String = values.iter().take(values.len() - 1).collect();
            println!("values: {:?}, number_val: {:?}", values, number_val);
            let number: i64 = number_val.trim().parse().expect("value did not parse");
            numbers.push(number);

            if !values.last().unwrap_or(&' ').is_whitespace() {
                problems.push(Problem {
                    numbers,
                    operation: Operation::from(*values.last().unwrap())
                });
                numbers = vec![];
            }
        }

        let result: i64 = problems
            .into_par_iter()
            .map(|problem| -> i64 {
                match problem.operation {
                    Operation::Add => problem.numbers.iter().sum(),
                    Operation::Multiply => problem.numbers.iter().product(),
                }
            })
            .sum();

        Ok(result)
    }
});

impl Day {}

struct Problem {
    numbers: Vec<i64>,
    operation: Operation,
}

#[derive(PartialEq)]
enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Not a valid value"),
        }
    }
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("Not a valid value"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day6-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 4277556;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 3263827;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
