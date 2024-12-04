#![allow(dead_code)]

use advent_2024::Year2024;
use common::year::{self, AdventYear};
use std::{env, io::stdin};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let year_to_run_option = args.get(1);
    let day_to_run_option = args.get(1);
    let year_to_run_option = args.get(2);

    if day_to_run_option.is_none() || year_to_run_option.is_none() {
        let _ = select_year(None, None);
    } else {
        let _ = select_year(
            year_to_run_option.unwrap().parse().ok(),
            day_to_run_option.unwrap().parse().ok(),
        );
    }

    // if let Some(day_to_run) = day_to_run_option {
    //     let running_day: i32 = day_to_run.parse().expect("arg must be an i32");
    //     let result = days::select_day(Some(running_day));
    //     if let Err(error_result) = result {
    //         println!("Error: {}", error_result);
    //     }
    // } else {
    //     let result = days::select_day(None);
    //     if let Err(error_result) = result {
    //         println!("Error: {}", error_result);
    //     }
    // }
}

fn select_year(
    input_year: Option<i32>,
    input_day: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(year) = input_year {
        return run_year(year, input_day);
    }

    let years_to_select = year::get_years();
    let first_year = years_to_select.first().unwrap();
    let last_year = years_to_select.last().unwrap();
    println!(
        "Please select a year to run ({}-{}). ({})",
        first_year, last_year, last_year
    );
    let mut selected_year = String::new();
    match stdin().read_line(&mut selected_year) {
        Ok(_) => {
            if selected_year.trim().is_empty() {
                run_year(last_year.to_owned(), input_day)
            } else {
                let year = selected_year.trim().parse::<i32>()?;
                run_year(year, input_day)
            }
        }
        Err(_) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Error reading input",
        ))),
    }
}

fn run_year(input_year: i32, input_day: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
    match input_year {
        2023 => Ok(()),
        2024 => Year2024.run(input_day),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Year not found",
        ))),
    }
}
