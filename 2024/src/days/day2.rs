use common::{day::AdventDay, file::read_file};
use std::{error::Error, fmt::Debug};

use crate::common_values::get_data_path;

pub struct Day2;
impl AdventDay for Day2 {
    fn puzzle1(&self, path: &str) -> Result<impl Debug, Box<dyn Error>> {
        let string = read_file(path).unwrap();
        let reports = string.lines();

        let mut safe_reports = 0;
        for report in reports {
            // check if all increase or decrease
            // check if increase/decrease is from 1-3

            let report_levels: Vec<i64> = report
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let results = eval_safety(&report_levels);

            if results.iter().all(|result| result.is_safe) {
                safe_reports += 1;
            }
        }

        Ok(safe_reports)
    }
    fn puzzle2(&self, path: &str) -> Result<impl Debug, Box<dyn Error>> {
        let string = read_file(path).unwrap();
        let reports = string.lines();

        let mut safe_reports = 0;
        for report in reports {
            // check if all increase or decrease
            // check if increase/decrease is from 1-3

            let report_levels: Vec<i64> = report
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            let results = eval_safety(&report_levels);

            if results.iter().all(|x| x.is_safe) {
                safe_reports += 1;
                continue;
            }

            let mut trial_success = false;
            for trial in 0..report_levels.len() - 1 {
                let mut trial_levels = report_levels.clone();
                trial_levels.remove(trial);
                let trial_results = eval_safety(&trial_levels);

                if trial_results.iter().all(|x| x.is_safe) {
                    println!(
                        "successful trial: {:?}. original: {:?}",
                        trial_levels, report_levels
                    );
                    trial_success = true;

                    break;
                } else {
                    println!("trial fail");
                }
            }

            if trial_success == true {
                safe_reports += 1;
                continue;
            } else {
                println!("all trials failed for: {:?}", report_levels);
            }
        }

        Ok(safe_reports)
    }

    fn run(&self) {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day2.txt";
        self.print_puzzles(2, day1_path.as_str(), day1_path.as_str());
    }
}

fn eval_safety(levels: &Vec<i64>) -> Vec<ReactorResult> {
    let mut results: Vec<ReactorResult> = Vec::new();
    let mut prev_value: Option<i64> = None;
    for (i, val) in levels.into_iter().enumerate() {
        let current = *val;
        if prev_value.is_none() {
            prev_value = Some(current);
            continue;
        }

        let mut increase = false;
        let mut decrease = false;
        if prev_value.unwrap() < current {
            increase = true;
        } else if prev_value.unwrap() > current {
            decrease = true;
        }

        let mut is_safe = true;

        if i >= 1 && results.len() > 0 {
            let prev_result = &results[i - 2];
            if prev_result.is_decrease != decrease && prev_result.is_increase != increase {
                println!("unsafe due to increase/decrease. report: {:?}", levels);
                is_safe = false;
            }
        }

        let mut diff = prev_value.unwrap() - current;
        if diff < 0 {
            diff *= -1;
        }
        if diff >= 1 && diff <= 3 {
            // safe
        } else {
            // unsafe
            println!(
                "report unsafe due to diff: {:?}. Values: {:?} {:?}",
                levels,
                prev_value.unwrap(),
                current
            );
            is_safe = false;
        }

        let result = ReactorResult {
            is_increase: increase,
            is_decrease: decrease,
            diff,
            is_safe,
            value: current,
        };
        results.push(result);

        prev_value = Some(current);
    }

    results
}

#[derive(Debug)]
struct ReactorResult {
    is_safe: bool,
    is_increase: bool,
    is_decrease: bool,
    diff: i64,
    value: i64,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = "data/day2-test.txt";
        let day = Day2;
        let result = day.puzzle1(path).unwrap();
        let expected: i64 = 2;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = "data/day2-test.txt";
        let day = Day2;
        let result = day.puzzle2(path).unwrap();
        let expected: i64 = 4;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
