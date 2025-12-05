use common::{file::process_file_into_lines, impl_day};

use std::path::Path;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(3, YEAR, "day3.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let battery_banks: Vec<BatteryBank> = lines
            .iter()
            .map(|line| {
                let batteries: Vec<Battery> = line
                .trim()
                .chars()
                .map(|battery| Battery::from(battery))
                .collect();

                BatteryBank::from(batteries)
            })
            .map(|battery_bank| Day::find_max_joltage(battery_bank))
            .collect();

        let mut output_joltage: i64 = 0;
        for bank in battery_banks {
            if let Some(joltage) = bank.max_joltage {
                output_joltage += joltage;
            }
        }

        Ok(output_joltage)
    },

    puzzle2: |_day: &Day, path: &Path| {
        Ok(())
    }
});

#[derive(Clone, Debug)]
struct Battery {
    joltage: i64,
}

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<Battery>,
    max_joltage: Option<i64>,
}

impl From<char> for Battery {
    fn from(value: char) -> Self {
        Battery {
            joltage: value
                .to_digit(10)
                .expect(&format!("failed to parse char {:?}", value)) as i64,
        }
    }
}

impl From<Vec<Battery>> for BatteryBank {
    fn from(value: Vec<Battery>) -> Self {
        BatteryBank {
            batteries: value,
            max_joltage: None,
        }
    }
}

impl Day {
    fn find_max_joltage(battery_bank: BatteryBank) -> BatteryBank {
        let batteries = &battery_bank.batteries;

        if batteries.is_empty() {
            return battery_bank;
        }

        let mut max_joltage = 0i64;

        // Find the largest digit
        let max_idx = batteries
            .iter()
            .enumerate()
            .max_by_key(|(_, b)| b.joltage)
            .map(|(idx, _)| idx)
            .unwrap();

        let max_digit = batteries[max_idx].joltage;

        // Try to find the largest digit after max_idx
        if max_idx + 1 < batteries.len() {
            let largest_after = batteries[max_idx + 1..]
                .iter()
                .max_by_key(|b| b.joltage)
                .unwrap()
                .joltage;
            max_joltage = max_digit * 10 + largest_after;
        } else if max_idx > 0 {
            // No digits after, so find largest before and put max_digit second
            let largest_before = batteries[..max_idx]
                .iter()
                .max_by_key(|b| b.joltage)
                .unwrap()
                .joltage;
            max_joltage = largest_before * 10 + max_digit;
        }

        let updated_bank = BatteryBank {
            batteries: batteries.to_vec(),
            max_joltage: Some(max_joltage),
        };

        #[cfg(test)]
        {
            println!("Bank: {:?}", updated_bank);
        }

        updated_bank
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day3-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 357;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 4174379265;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
