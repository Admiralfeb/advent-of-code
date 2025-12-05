use common::{file::read_file, impl_day};
use std::{error::Error, path::Path};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(1, YEAR, "day1.txt", {
    puzzle1: |day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        let mut lists = day.make_lists(string)?;

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
    },
    puzzle2: |day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        let lists = day.make_lists(string)?;

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
    },
});

struct Lists {
    left_list: Vec<i64>,
    right_list: Vec<i64>,
}

impl Day {
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
    use common::{day::AdventDay, file::get_data_path};

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, "day1-test.txt");

        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 11;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, "day1-test.txt");

        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 31;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
