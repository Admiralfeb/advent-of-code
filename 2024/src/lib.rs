#![allow(dead_code)]

use common::year::AdventYear;

mod common_values;
pub mod days;

pub struct Year2024;

impl AdventYear for Year2024 {
    fn run(&self, input_day: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let result = days::select_day(input_day)?;

        println!("year 2024 {}", result);
        Ok(())
    }
}

impl Year2024 {}
