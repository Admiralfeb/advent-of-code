use common::impl_year;

mod common_values;
pub mod days;

pub struct Year2023;

impl_year!(Year2023, 2023, {
    unit: [
        1 => day01,
        2 => day02,
    ]
    default: []
});
