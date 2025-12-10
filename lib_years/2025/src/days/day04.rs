use common::{file::process_file_into_lines, impl_day};

use std::collections::HashSet;
use std::path::Path;

use crate::common_values::YEAR;

pub struct Day;

fn count_surrounding_rolls(location: &Point, roll_locations: &HashSet<Point>) -> usize {
    let mut count = 0;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            // Skip the center point (0, 0)
            if dx == 0 && dy == 0 {
                continue;
            }

            // Calculate new coordinates with bounds checking
            if let (Some(new_x), Some(new_y)) = (
                location.x.checked_add_signed(dx),
                location.y.checked_add_signed(dy),
            ) {
                if roll_locations.contains(&Point { x: new_x, y: new_y }) {
                    count += 1;
                }
            }
        }
    }

    count
}

impl_day!(4, YEAR, "day04.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        // Build a HashSet of all roll locations for O(1) lookup
        let roll_locations: HashSet<Point> = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.char_indices()
                    .filter(|c| c.1 == '@')
                    .map(move |(j, _)| Point { x: j, y: i })
            })
            .collect();

        #[cfg(test)]
        {
            println!("total rolls: {}", roll_locations.len());
        }

        // Count accessible rolls directly without storing mutable state
        let accessible_rolls = roll_locations
            .iter()
            .filter(|&location| {
                let surrounding_count = count_surrounding_rolls(location, &roll_locations);
                #[cfg(test)]
                {
                    println!("surrounding rolls for roll {:?}: {}", location, surrounding_count);
                }
                surrounding_count < 4
            })
            .count();

        Ok(accessible_rolls)
    },

    puzzle2: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let mut removeable_rolls = 0;
        // Build a HashSet of all roll locations for O(1) lookup
        let mut roll_locations: HashSet<Point> = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.char_indices()
                    .filter(|c| c.1 == '@')
                    .map(move |(j, _)| Point { x: j, y: i })
            })
            .collect();

        loop {
            let accessible_count = recursively_remove_rolls(&mut roll_locations, &mut removeable_rolls);
            if accessible_count == 0 {
                break;
            }
        }

        Ok(removeable_rolls)
    }
});

fn recursively_remove_rolls(locations: &mut HashSet<Point>, removeable_rolls: &mut i32) -> usize {
    // Count accessible rolls directly without storing mutable state
    let accessible_rolls: Vec<Point> = locations
        .iter()
        .filter(|&location| {
            let surrounding_count = count_surrounding_rolls(location, &locations);
            #[cfg(test)]
            {
                println!(
                    "surrounding rolls for roll {:?}: {}",
                    location, surrounding_count
                );
            }
            surrounding_count < 4
        })
        .copied()
        .collect();

    let count = accessible_rolls.len();
    *removeable_rolls += count as i32;

    // Remove the accessible rolls from the set
    for roll in accessible_rolls {
        locations.remove(&roll);
    }

    count
}

impl Day {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day04-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 13;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i64 = 43;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
