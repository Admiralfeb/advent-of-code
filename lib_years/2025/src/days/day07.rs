use common::{file::process_file_into_lines, impl_day};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use regex::Regex;

use std::{path::Path, sync::LazyLock};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(7, YEAR, "day07.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;


        Ok(())
    },

    puzzle2: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;


        Ok(())
    }
});

impl Day {}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day07-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 21;
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
