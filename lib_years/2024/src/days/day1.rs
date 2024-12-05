use common::{day::AdventDay, file::read_file};
use std::{error::Error, fmt};

use crate::common_values::get_data_path;

pub struct Day1;
impl AdventDay for Day1 {
    fn puzzle1(&self, path: &str) -> Result<impl fmt::Debug, Box<dyn Error>> {
        let string = read_file(path).unwrap();

        let mut lists = self.make_lists(string)?;

        lists.left_list.sort();
        lists.right_list.sort();

        let mut main_delta: i64 = 0;

        for i in 0..lists.left_list.len() {
            let delta = lists.right_list[i] - lists.left_list[i];
            if delta < 0 {
                main_delta += delta * -1;
            } else {
                main_delta += delta;
            }
        }

        Ok(main_delta)
    }

    fn puzzle2(&self, path: &str) -> Result<impl fmt::Debug, Box<dyn Error>> {
        let string = read_file(path).unwrap();

        let lists = self.make_lists(string)?;

        let mut similarities: i64 = 0;

        for i in lists.left_list {
            let count: i64 = lists
                .right_list
                .iter()
                .filter(|x| **x == i)
                .count()
                .try_into()
                .unwrap();
            similarities += i * count;
        }

        Ok(similarities)
    }
    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day1.txt";
        self.print_puzzles(1, day1_path.as_str(), day1_path.as_str())
    }
}

struct Lists {
    left_list: Vec<i64>,
    right_list: Vec<i64>,
}

impl Day1 {
    fn make_lists(&self, string: String) -> Result<Lists, Box<dyn Error>> {
        let lines = string.lines();
        let mut left_list: Vec<i64> = Vec::new();
        let mut right_list: Vec<i64> = Vec::new();

        for line in lines {
            let mut split = line.split_ascii_whitespace();
            let left = split.next().unwrap().parse::<i64>().unwrap();
            let right = split.next().unwrap().parse::<i64>().unwrap();
            left_list.push(left);
            right_list.push(right);
        }
        let lists = Lists {
            left_list,
            right_list,
        };
        Ok(lists)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = "data/day1-test.txt";
        let day = Day1;
        let result = day.puzzle1(path).unwrap();
        let expected: i64 = 11;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = "data/day1-test.txt";
        let day = Day1;
        let result = day.puzzle2(path).unwrap();
        let expected: i64 = 31;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
