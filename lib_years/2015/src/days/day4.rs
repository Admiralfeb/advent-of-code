use std::path::Path;

use common::impl_day;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(4, YEAR, "day4.txt", {
    puzzle1: |_day: &Day, _path: &Path| {
        let string = "yzbqklnj";
        let result = Day::calc_hash(&string, 5);
        Ok(result)
    },
    puzzle2: |_day: &Day, _path: &Path| {
        let string = "yzbqklnj";
        let result = Day::calc_hash(&string, 6);
        Ok(result)
    },
});

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
