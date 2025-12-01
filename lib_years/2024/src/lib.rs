use common::impl_year;

mod common_values;
mod days;

pub struct Year2024;

impl_year!(Year2024, 2024, {
    unit: [
        1 => day1,
        2 => day2,
        3 => day3,
        4 => day4,
        5 => day5,
        9 => day9,
    ]
    default: []
});
