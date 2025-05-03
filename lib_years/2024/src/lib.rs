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
            1 => Ok(day1::Day1.run()),
            2 => Ok(day2::Day2.run()),
            3 => Ok(day3::Day3.run()),
            4 => Ok(day4::Day4.run()),
            5 => Ok(day5::Day5.run()),
            6 => Ok(day6::Day6.run()),
            7 => Ok(day7::Day7.run()),
            8 => Ok(day8::Day8.run()),
            9 => Ok(day9::Day9.run()),
            10 => Ok(day10::Day10.run()),
            11 => Ok(day11::Day11.run()),
            12 => Ok(day12::Day12.run()),
            13 => Ok(day13::Day13.run()),
            14 => Ok(day14::Day14.run()),
            15 => Ok(day15::Day15.run()),
            16 => Ok(day16::Day16.run()),
            17 => Ok(day17::Day17.run()),
            18 => Ok(day18::Day18.run()),
            19 => Ok(day19::Day19.run()),
            20 => Ok(day20::Day20.run()),
            21 => Ok(day21::Day21.run()),
            22 => Ok(day22::Day22.run()),
            23 => Ok(day23::Day23.run()),
            24 => Ok(day24::Day24.run()),
            25 => Ok(day25::Day25.run()),
            _ => Err(Box::new(DayNotFoundError)),
        }
    }
}
