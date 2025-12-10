use common::impl_year;

mod common_values;
pub mod days;

pub struct Year2025;

impl_year!(Year2025, 2025, {
    unit: [
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
        5 => day05,
        6 => day06,
        7 => day07
    ]
    default: []
});
