use common::impl_year;

mod common_values;
pub mod days;

pub struct Year2023;

impl_year!(Year2023, 2023, {
    unit: [
        1 => day1,
        2 => day2,
    ]
    default: []
});
