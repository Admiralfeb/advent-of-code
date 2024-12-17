use common::{day::AdventDay, file::read_file};
use std::{error::Error, fmt};

use crate::common_values::get_data_path;

pub struct Day21;
impl AdventDay for Day21 {
    fn puzzle1(&self, path: &str) -> Result<impl fmt::Debug, Box<dyn Error>> {
        let binding = read_file(path)?;
        Ok("")
    }

    fn puzzle2(&self, path: &str) -> Result<impl fmt::Debug, Box<dyn Error>> {
        Ok("")
    }
    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day21.txt";
        self.print_puzzles(6, day1_path.as_str(), day1_path.as_str())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // #[test]
    fn test_puzzle1() {
        let path = "data/day21-test.txt";
        let day = Day21;
        let result = day.puzzle1(path).unwrap();
        let expected: i64 = 11;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    // #[test]
    fn test_puzzle2() {
        let path = "data/day21-test.txt";
        let day = Day21;
        let result = day.puzzle2(path).unwrap();
        let expected: i64 = 31;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
