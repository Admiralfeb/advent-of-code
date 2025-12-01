use std::path::Path;

use common::{file::read_file, impl_day};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(7, YEAR, "day6.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let _file_string = read_file(path).unwrap();
        // let lights_on = self.process_data(file_string);
        Ok("lights_on")
    },
    puzzle2: |_day: &Day, path: &Path| {
        let _file_string = read_file(path).unwrap();
        // let total_brightness = self.process_data_p2(file_string);
        Ok("total_brightness")
    },
});

impl Day {
    fn process_data() {
        let _num: u16 = 0;
    }
}
