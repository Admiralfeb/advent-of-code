#![allow(dead_code)]

use std::error;

use common::{
    day::AdventDay,
    year::{AdventYear, DayNotFoundError},
};

mod common_values;
pub mod days;
use days::*;

pub struct Year2024;

impl AdventYear for Year2024 {
    fn run(&self, input_day: Option<i32>) -> Result<(), Box<dyn error::Error>> {
        let result = self.select_day(input_day)?;

        println!("year 2024 {}", result);
        Ok(())
    }
    fn get_days(&self) -> Vec<i32> {
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]
    }
    fn run_day(&self, input: i32) -> Result<String, Box<dyn error::Error>> {
        match input {
            1 => Ok(day1::Day.run()),
            2 => Ok(day2::Day.run()),
            3 => Ok(day3::Day.run()),
            4 => Ok(day4::Day.run()),
            5 => Ok(day5::Day.run()),
            9 => Ok(day9::Day.run()),
            _ => Err(Box::new(DayNotFoundError)),
        }
    }
}
