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
        vec![1, 2, 3, 4, 5]
    }
    fn run_day(&self, input: i32) -> Result<String, Box<dyn error::Error>> {
        match input {
            1 => Ok(day1::Day1.run()),
            2 => Ok(day2::Day2.run()),
            3 => Ok(day3::Day3.run()),
            4 => Ok(day4::Day4.run()),
            5 => Ok(day5::Day5.run()),
            _ => Err(Box::new(DayNotFoundError)),
        }
    }
}
