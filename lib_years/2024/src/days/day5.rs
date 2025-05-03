use common::{day::AdventDay, file::read_file};
use std::{error::Error, fmt};

use crate::common_values::get_data_path;

pub struct Day5;
impl AdventDay for Day5 {
    fn puzzle1(&self, path: &str) -> Result<impl fmt::Debug, Box<dyn Error>> {
        let (page_rules, page_updates) = process_data(path)?;

        let mut total: i64 = 0;
        for update in page_updates {
            // check if both pages are present
            let mut bad_update: bool = false;
            for rule in &page_rules {
                if !update.contains(&rule.first_page) || !update.contains(&rule.second_page) {
                    continue;
                }
                // if they are, confirm that they are in the right order
                let first_index = update.iter().position(|u| u == &rule.first_page).unwrap();
                let second_index = update.iter().position(|u| u == &rule.second_page).unwrap();
                if first_index > second_index {
                    bad_update = true;
                    break;
                }
            }
            // if they're in the right order, then grab and sum the middle number
            if !bad_update {
                continue;
            }

            // reorg

            let middle_index = (update.len() as f32) / 2.0;
            let middle_number = update[middle_index as usize];
            println!("middle_number: {}", middle_number);
            total += middle_number as i64;
        }

        Ok(total)
    }

    fn puzzle2(&self, path: &str) -> Result<impl fmt::Debug, Box<dyn Error>> {
        let (page_rules, page_updates) = process_data(path)?;

        let mut total: i64 = 0;
        for update in page_updates {
            // check if both pages are present
            let mut bad_update: bool = false;
            for rule in &page_rules {
                if !update.contains(&rule.first_page) || !update.contains(&rule.second_page) {
                    continue;
                }
                // if they are, confirm that they are in the right order
                let first_index = update.iter().position(|u| u == &rule.first_page).unwrap();
                let second_index = update.iter().position(|u| u == &rule.second_page).unwrap();
                if first_index > second_index {
                    bad_update = true;
                    break;
                }
            }
            // if they're in the right order, then grab and sum the middle number
            if bad_update {
                continue;
            }

            let middle_index = (update.len() as f32) / 2.0;
            let middle_number = update[middle_index as usize];
            println!("middle_number: {}", middle_number);
            total += middle_number as i64;
        }

        Ok(total)
    }
    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day5.txt";
        self.print_puzzles(5, day1_path.as_str(), day1_path.as_str())
    }
}

fn process_data(path: &str) -> Result<(Vec<PageRule>, Vec<Vec<i32>>), Box<dyn Error>> {
    let data = read_file(path)?;
    let mut data_split = data.split("\n\n");
    let page_rules = data_split
        .next()
        .unwrap()
        .split("\n")
        .map(|rule| {
            let s: Vec<&str> = rule.split('|').collect();
            let first = s[0].parse::<i32>().unwrap();
            let second = s[1].parse::<i32>().unwrap();
            PageRule {
                first_page: first,
                second_page: second,
            }
        })
        .collect::<Vec<PageRule>>();
    let page_updates = data_split
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|update| {
            update
                .split(',')
                .map(|u| u.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Ok((page_rules, page_updates))
}

#[derive(Clone, Copy)]
struct PageRule {
    first_page: i32,
    second_page: i32,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = "data/day5-test.txt";
        let day = Day5;
        let result = day.puzzle1(path).unwrap();
        let expected: i64 = 143;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = "data/day5-test.txt";
        let day = Day5;
        let result = day.puzzle2(path).unwrap();
        let expected: i64 = 123;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
