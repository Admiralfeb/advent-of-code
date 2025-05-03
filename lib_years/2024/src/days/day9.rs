use common::{
    day::AdventDay,
    file::{get_data_path, read_file},
};
use std::{error::Error, fmt, path::Path};

use crate::common_values::YEAR;

pub struct Day;
impl AdventDay for Day {
    fn puzzle1(&self, path: &Path) -> Result<impl fmt::Debug, Box<dyn Error>> {
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
    }

    fn puzzle2(&self, path: &Path) -> Result<impl fmt::Debug, Box<dyn Error>> {
        Ok("")
    }
    fn run(&self) -> String {
        let data_path = get_data_path(YEAR, "day9.txt");
        self.print_puzzles(6, &data_path, &data_path)
    }
}

#[cfg(test)]
mod test {

    use super::*;

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
    fn test_puzzle2() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 31;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
