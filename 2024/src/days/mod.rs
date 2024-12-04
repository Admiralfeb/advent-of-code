use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::stdin,
};

use common::day::AdventDay;

pub mod day1;
pub mod day2;
pub mod day3;

fn run_day(input: i32) -> Result<String, Box<dyn Error>> {
    match input {
        1 => Ok(day1::Day1.run()),
        2 => Ok(day2::Day2.run()),
        3 => Ok(day3::Day3.run()),
        _ => Err(Box::new(DayNotFoundError)),
    }
}

pub fn select_day(input: Option<i32>) -> Result<String, Box<dyn Error>> {
    if let Some(input_day) = input {
        return run_day(input_day);
    }

    let days_to_select = vec![1, 2, 3];
    println!(
        "Please select a day to run (1-{})",
        days_to_select.last().unwrap()
    );
    let mut selected_day = String::new();
    match stdin().read_line(&mut selected_day) {
        Ok(_) => {
            let day = selected_day.trim().parse::<i32>()?;
            run_day(day)
        }
        Err(_) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Error reading input",
        ))),
    }
}

#[derive(Debug)]
struct DayNotFoundError;

impl Error for DayNotFoundError {}

impl Display for DayNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Day not found. Select another day.")
    }
}
