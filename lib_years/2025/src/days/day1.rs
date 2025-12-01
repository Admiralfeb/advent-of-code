use common::{file::read_file, impl_day};
use std::path::Path;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(1, YEAR, "day1.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let _string = read_file(path).unwrap();

        Ok("result")
    },

    puzzle2: |_day: &Day, path: &Path| {
        let _string = read_file(path).unwrap();
        Ok("result")
    }
});
