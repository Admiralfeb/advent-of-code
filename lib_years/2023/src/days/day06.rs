use std::{error::Error, fs, num::ParseIntError, path::Path, time::Instant};

use common::impl_day;

use crate::common_values::YEAR;

pub struct Day;

struct Race {
    time: i64,
    distance: i64,
}

impl_day!(6, YEAR, "day06-input.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let start = Instant::now();
        let result = Day::puzzle1(path)?;
        println!("Day 6 Puzzle 1 took: {:?}", start.elapsed());
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let start = Instant::now();
        let result = Day::puzzle2(path)?;
        println!("Day 6 Puzzle 2 took: {:?}", start.elapsed());
        Ok(result)
    },
});

impl Day {
    fn puzzle1(path: &Path) -> Result<i64, Box<dyn Error>> {
        let input_value = fs::read_to_string(path)?;

        let mut races: Vec<Race> = Vec::new();
        let mut inputs = input_value.lines();

        let times = parse_nums(inputs.next().unwrap()).unwrap();
        let distances = parse_nums(inputs.next().unwrap()).unwrap();

        for i in 0..times.len() {
            races.push(Race {
                time: times[i],
                distance: distances[i],
            });
        }

        let mut total_winners: i64 = 1;
        for race in races {
            let mut winners = 0;
            for i in 0..race.time {
                let remaining_time = race.time - i;
                let distance_travelled = i * remaining_time;
                if distance_travelled > race.distance {
                    winners += 1;
                }
            }
            total_winners *= winners;
        }

        Ok(total_winners)
    }

    fn puzzle2(path: &Path) -> Result<i64, Box<dyn Error>> {
        let input_value = fs::read_to_string(path)?;

        let mut inputs = input_value.lines();

        let time = parse_nums2(inputs.next().unwrap()).unwrap();
        let distance = parse_nums2(inputs.next().unwrap()).unwrap();

        // We need to find the range where: i * (time - i) > distance
        // Expanding: -i^2 + time*i - distance > 0
        // Or rearranged: i^2 - time*i + distance < 0
        // Using quadratic formula: i = (time ± sqrt(time^2 - 4*distance)) / 2

        let time_f = time as f64;
        let distance_f = distance as f64;

        let discriminant = time_f * time_f - 4.0 * distance_f;
        if discriminant < 0.0 {
            return Ok(0); // No solutions
        }

        let sqrt_discriminant = discriminant.sqrt();
        let lower = (time_f - sqrt_discriminant) / 2.0;
        let upper = (time_f + sqrt_discriminant) / 2.0;

        // We need strict inequality (>), so adjust the bounds
        // Lower bound: ceil if it's exactly on the boundary, otherwise floor + 1
        // Upper bound: floor if it's exactly on the boundary, otherwise ceil - 1
        let lower_bound = if lower == lower.floor() {
            lower as i64 + 1
        } else {
            lower.ceil() as i64
        };

        let upper_bound = if upper == upper.floor() {
            upper as i64 - 1
        } else {
            upper.floor() as i64
        };

        let winners = upper_bound - lower_bound + 1;

        Ok(winners)
    }
}

fn parse_nums(input: &str) -> Result<Vec<i64>, ParseIntError> {
    let val = input.split(":").nth(1).unwrap();
    let updated_val: Vec<i64> = val
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<i64>>();
    Ok(updated_val)
}
fn parse_nums2(input: &str) -> Result<i64, ParseIntError> {
    let val = input.split(":").nth(1).unwrap();
    val.replace(" ", "").trim().parse::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use common::file::get_data_path;

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, "day06-test-input.txt");
        let result = Day::puzzle1(&path);

        assert_eq!(288, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, "day06-test-input.txt");
        let result = Day::puzzle2(&path);

        assert_eq!(71503, result.unwrap())
    }
}
