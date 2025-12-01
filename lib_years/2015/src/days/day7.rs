use std::path::Path;

use common::{
    day::AdventDay,
    file::{get_data_path, read_file},
};
use grid::Grid;

use crate::common_values::YEAR;

pub struct Day;

impl AdventDay for Day {
    fn run(&self) -> String {
        let data_path = get_data_path(YEAR, "day6.txt");
        self.print_puzzles(7, data_path.as_path(), data_path.as_path())
    }

    fn puzzle1(&self, path: &Path) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let file_string = read_file(path).unwrap();

        // let lights_on = self.process_data(file_string);

        Ok("lights_on")
    }

    fn puzzle2(&self, path: &Path) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let file_string = read_file(path).unwrap();

        // let total_brightness = self.process_data_p2(file_string);

        Ok("total_brightness")
    }
}

impl Day {
    fn process_data() {
        let mut num: u16 = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
}
