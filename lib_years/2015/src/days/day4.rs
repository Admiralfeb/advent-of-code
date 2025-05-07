use std::path::Path;

use common::{day::AdventDay, file::get_data_path};

use crate::common_values::YEAR;

pub struct Day;
impl AdventDay for Day {
    fn run(&self) -> String {
        let data_path = get_data_path(YEAR, "day4.txt");
        self.print_puzzles(4, data_path.as_path(), data_path.as_path())
    }

    fn puzzle1(&self, _path: &Path) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let string = "yzbqklnj";

        let result = Day::calc_hash(&string, 5);
        Ok(result)
    }

    fn puzzle2(&self, _path: &Path) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let string = "yzbqklnj";

        let result = Day::calc_hash(&string, 6);
        Ok(result)
    }
}

impl Day {
    fn calc_hash(input: &str, num_zeros: usize) -> i64 {
        let mut i: i64 = 0;
        loop {
            i += 1;
            let digest = md5::compute(format!("{}{}", input, i));
            let d_string = format!("{:?}", digest);
            if d_string.starts_with(&"0".repeat(num_zeros)) {
                return i;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdef" => 609043)]
    #[test_case("pqrstuv" => 1048970)]
    fn test_calc_hash(input: &str) -> i64 {
        Day::calc_hash(input, 5)
    }
}
