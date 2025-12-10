use common::impl_year;

mod common_values;
mod days;

pub struct Year2024;

impl_year!(Year2024, 2024, {
    unit: [
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
        5 => day05,
        9 => day09,
    ]
    default: []
});
