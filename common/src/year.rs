use std::{error, fmt, io};

pub fn get_years() -> Vec<i32> {
    vec![2023, 2024]
}

pub trait AdventYear {
    fn run(&self, input: Option<i32>) -> Result<(), Box<dyn error::Error>>;
    fn select_day(&self, input: Option<i32>) -> Result<String, Box<dyn error::Error>> {
        if let Some(input_day) = input {
            return self.run_day(input_day);
        }

        let days_to_select = self.get_days();
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
pub struct DayNotFoundError;

impl error::Error for DayNotFoundError {}

impl fmt::Display for DayNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day not found. Select another day.")
    }
}
