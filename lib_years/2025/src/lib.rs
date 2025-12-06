use common::impl_year;

mod common_values;
pub mod days;

pub struct Year2025;

impl_year!(Year2025, 2025, {
    unit: [
        1 => day1,
        2 => day2,
        3 => day3,
        4 => day4,
        5 => day5
    ]
    default: []
});
