use common::{file::process_file_into_lines, impl_day};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use regex::Regex;

use std::{collections::HashMap, path::Path, sync::LazyLock};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(8, YEAR, "day8.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let boxes: Vec<JunctionBox> = lines
            .iter()
            .map(|line| line.into())
            .collect();



        Ok(())
    },

    puzzle2: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;


        Ok(())
    }
});

impl Day {}

#[derive(PartialEq, Eq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn closest_from_list<'a>(
        &self,
        boxes: &'a Vec<JunctionBox>,
        ignored_boxes: &Vec<JunctionBox>,
    ) -> &'a JunctionBox {
        boxes
            .par_iter()
            .filter(|b| !ignored_boxes.contains(b) && *b != self)
            .min_by(|a, b| {
                let dist_a = self.get_distance(a);
                let dist_b = self.get_distance(b);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .expect("should have found a box")
    }

    fn get_distance(&self, other: &JunctionBox) -> f64 {
        (((other.x - self.x) ^ 2 + (other.y - self.y) ^ 2 + (other.z - self.z) ^ 2) as f64).sqrt()
    }
}

impl From<&String> for JunctionBox {
    fn from(value: &String) -> Self {
        let splits: Vec<i64> = value
            .split(',')
            .into_iter()
            .map(|s| s.parse().expect(&format!("Failed to parse value: {}", s)))
            .collect();

        JunctionBox {
            x: splits[0],
            y: splits[1],
            z: splits[2],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day8-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 40;
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
