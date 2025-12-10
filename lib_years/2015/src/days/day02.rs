use std::{error::Error, path::Path};

use common::{file::read_file, impl_day};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(2, YEAR, "day02.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        Day::process_data(string, Day::calc_wrapping_paper)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        Day::process_data(string, Day::calc_ribbon)
    },
});

impl Day {
    fn process_data(
        string: String,
        func: impl Fn(i32, i32, i32) -> i32,
    ) -> Result<i32, Box<dyn Error + 'static>> {
        let mut result: i32 = 0;
        for line in string.lines() {
            let values: Vec<i32> = line.split('x').map(|v| v.parse::<i32>().unwrap()).collect();
            let length = values[0];
            let width = values[1];
            let height = values[2];

            result += func(length, width, height);
        }

        Ok(result)
    }

    /** Calculate wrapping paper needed for elves */
    fn calc_wrapping_paper(length: i32, width: i32, height: i32) -> i32 {
        let lw = length * width;
        let wh = width * height;
        let lh = length * height;
        let min_val = vec![lw, wh, lh].into_iter().min().unwrap();

        2 * lw + 2 * wh + 2 * lh + min_val
    }

    fn calc_ribbon(length: i32, width: i32, height: i32) -> i32 {
        let mut num_list = vec![length, width, height];
        num_list.sort();
        let smallest_nums: Vec<i32> = num_list.into_iter().take(2).collect();

        2 * smallest_nums[0] + 2 * smallest_nums[1] + length * width * height
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(2,3,4 => 58)]
    #[test_case(1,1,10 => 43)]
    fn test_calc_wrapping_paper(length: i32, width: i32, height: i32) -> i32 {
        Day::calc_wrapping_paper(length, width, height)
    }

    #[test_case(2,3,4 => 34)]
    #[test_case(1,1,10 => 14)]
    fn test_calc_ribbon(length: i32, width: i32, height: i32) -> i32 {
        Day::calc_ribbon(length, width, height)
    }
}
