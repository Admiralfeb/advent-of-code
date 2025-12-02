use common::{file::read_file, impl_day};
use std::path::Path;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(1, YEAR, "day1.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        println!("data file path: {:?}", path);

        let string = read_file(path).unwrap();
        let lines = string.lines();

        let commands = lines
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: (&str, &str) = line.split_at(1);
                let direction = match parts.0 {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Unknown direction"),
                };
                let value: i32 = parts.1.parse().unwrap_or_else(|_| panic!("Failed to parse '{}' as rotation value in line: {}", parts.1, line));
                RotationCommand { direction, value }
            })
            .collect::<Vec<RotationCommand>>();

        let result = Day::calc_number_of_times_0(50, commands);

        Ok(result)
    },

    puzzle2: |_day: &Day, path: &Path| {
        println!("data file path: {:?}", path);

        let string = read_file(path).unwrap();
        let lines = string.lines();

        let commands = lines
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: (&str, &str) = line.split_at(1);
                let direction = match parts.0 {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Unknown direction"),
                };
                let value: i32 = parts.1.parse().unwrap_or_else(|_| panic!("Failed to parse '{}' as rotation value in line: {}", parts.1, line));
                RotationCommand { direction, value }
            })
            .collect::<Vec<RotationCommand>>();

        let result = Day::calc_number_of_times_passes_0(50, commands);

        Ok(result)
    }
});

impl Day {
    fn calc_number_of_times_0(initial_position: i32, commands: Vec<RotationCommand>) -> i64 {
        let mut position = initial_position;
        let mut count_0 = 0;

        for command in commands {
            match command.direction {
                Direction::Left => {
                    position = (position - command.value).rem_euclid(100);
                }
                Direction::Right => {
                    position = (position + command.value).rem_euclid(100);
                }
            }

            if position == 0 {
                count_0 += 1;
            }
        }

        count_0
    }

    fn calc_number_of_times_passes_0(initial_position: i32, commands: Vec<RotationCommand>) -> i64 {
        let mut position = initial_position;
        let mut count_0 = 0;

        for command in commands {
            let old_position = position;

            // Calculate new position
            position = match command.direction {
                Direction::Left => (old_position - command.value).rem_euclid(100),
                Direction::Right => (old_position + command.value).rem_euclid(100),
            };

            // Count how many times we pass through 0 (including if we land on it)
            // BUT: if we start at 0, don't count leaving it
            let times_hit_0 = match command.direction {
                Direction::Left => {
                    // Going left: count how many complete cycles we make through 0
                    if old_position == 0 {
                        // Starting at 0, going left - count how many times we come back to 0
                        command.value / 100
                    } else if command.value >= old_position {
                        // We'll hit 0 at least once
                        1 + ((command.value - old_position) / 100)
                    } else {
                        0
                    }
                }
                Direction::Right => {
                    // Going right: count how many times we pass 100 (which wraps to 0)
                    if old_position == 0 {
                        // Starting at 0, going right - count how many times we come back to 0
                        command.value / 100
                    } else {
                        let total = old_position + command.value;
                        if total >= 100 {
                            1 + ((total - 100) / 100)
                        } else {
                            0
                        }
                    }
                }
            };

            count_0 += times_hit_0 as i64;
        }

        count_0
    }
}

struct RotationCommand {
    direction: Direction,
    value: i32,
}
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day1-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 3;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 6;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
