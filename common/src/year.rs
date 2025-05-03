use std::{error, fmt, io};

pub fn get_years() -> Vec<i32> {
    vec![2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024]
}

pub trait AdventYear {
    fn run(&self, input: Option<i32>) -> Result<(), Box<dyn error::Error>>;
    fn select_day(&self, input: Option<i32>) -> Result<String, Box<dyn error::Error>> {
        let days_to_select = self.get_days();
        if days_to_select.len() == 0 {
            return Err(Box::new(NoDaysError));
        }

        if let Some(input_day) = input {
            return self.run_day(input_day);
        }

        println!(
            "Please select a day to run (1-{})",
            days_to_select.last().unwrap()
        );
        let mut selected_day = String::new();
        match io::stdin().read_line(&mut selected_day) {
            Ok(_) => {
                let day = selected_day.trim().parse::<i32>()?;
                self.run_day(day)
            }
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Error reading input",
            ))),
        }
    }
    fn get_days(&self) -> Vec<i32>;
    fn run_day(&self, input: i32) -> Result<String, Box<dyn error::Error>>;
}

#[derive(Debug)]
pub struct NoDaysError;

impl error::Error for NoDaysError {}

impl fmt::Display for NoDaysError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No days present for this year.")
    }
}

#[derive(Debug)]
pub struct DayNotFoundError;

impl error::Error for DayNotFoundError {}

impl fmt::Display for DayNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day not found. Select another day.")
    }
}
