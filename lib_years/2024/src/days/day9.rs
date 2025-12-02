use common::{file::read_file, impl_day};
use std::path::Path;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(9, YEAR, "day9.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let binding = read_file(path)?;

        let mut block_num = 0;
        let mut expanded_string = String::new();
        for (index, c) in binding.chars().enumerate() {
            let value = c.to_digit(10).unwrap();

            if index % 2 != 0 {
                for _v in 0..value {
                    expanded_string += ".";
                }
            } else {
                for _v in 0..value {
                    expanded_string.push_str(block_num.to_string().as_str());
                }
                block_num += 1;
            }
        }

        Ok("")
    },
    puzzle2: |_day: &Day, _path: &Path| {
        Ok("")
    },
});

#[cfg(test)]
mod test {

    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day9-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 1928;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    // #[test]
    #[allow(dead_code)]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 31;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
