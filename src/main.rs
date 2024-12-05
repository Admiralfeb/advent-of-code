#![allow(dead_code)]

use advent_2015::Year2015;
use advent_2016::Year2016;
use advent_2017::Year2017;
use advent_2018::Year2018;
use advent_2019::Year2019;
use advent_2020::Year2020;
use advent_2021::Year2021;
use advent_2022::Year2022;
use advent_2023::Year2023;
use advent_2024::Year2024;
use common::year::{self, AdventYear};
use std::{env, error::Error, io::stdin};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let year_to_run_option = args.get(1);
    let day_to_run_option = args.get(1);
    let year_to_run_option = args.get(2);

    let result: Result<(), Box<dyn Error>>;
    if day_to_run_option.is_none() || year_to_run_option.is_none() {
        result = select_year(None, None);
    } else {
        result = select_year(
            year_to_run_option.unwrap().parse().ok(),
            day_to_run_option.unwrap().parse().ok(),
        );
    }
    if let Err(e) = result {
        println!("Error: {}", e);
    }
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
        2015 => Year2015.run(input_day),
        2016 => Year2016.run(input_day),
        2017 => Year2017.run(input_day),
        2018 => Year2018.run(input_day),
        2019 => Year2019.run(input_day),
        2020 => Year2020.run(input_day),
        2021 => Year2021.run(input_day),
        2022 => Year2022.run(input_day),
        2023 => Year2023.run(input_day),
        2024 => Year2024.run(input_day),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Year not found",
        ))),
    }
}
