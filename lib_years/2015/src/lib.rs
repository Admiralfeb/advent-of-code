#![allow(dead_code)]

use common::impl_year;

mod common_values;
pub mod days;

pub struct Year2015;

impl_year!(Year2015, 2015, {
    unit: [
        1 => day1,
        2 => day2,
        3 => day3,
        4 => day4,
        5 => day5,
        7 => day7,
    ]
    default: [
        6 => day6,
    ]
});
