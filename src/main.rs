#![allow(dead_code)]

use advent_2024::days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let year_to_run_option = args.get(1);
    let day_to_run_option = args.get(1);
    if let Some(day_to_run) = day_to_run_option {
        let running_day: i32 = day_to_run.parse().expect("arg must be an i32");
        let result = days::select_day(Some(running_day));
        if let Err(error_result) = result {
            println!("Error: {}", error_result);
        }
    } else {
        let result = days::select_day(None);
        if let Err(error_result) = result {
            println!("Error: {}", error_result);
        }
    }
}
