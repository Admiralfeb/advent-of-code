use std::error;

use common::{
    day::AdventDay,
    year::{AdventYear, DayNotFoundError},
};

mod common_values;
pub mod days;
use days::*;

pub struct Year2023;

impl AdventYear for Year2023 {
    fn run(&self, input_day: Option<i32>) -> Result<(), Box<dyn error::Error>> {
        let result = self.select_day(input_day)?;

        println!("year 2023 {}", result);
        Ok(())
    }
    fn get_days(&self) -> Vec<i32> {
        vec![]
    }
    fn run_day(&self, input: i32) -> Result<String, Box<dyn error::Error>> {
        match input {
            1 => Ok(day1::Day1.run()),
            // 2 => Ok(day2::Day2.run()),
            // 3 => Ok(day3::Day3.run()),
            _ => Err(Box::new(DayNotFoundError)),
        }
    }
}
