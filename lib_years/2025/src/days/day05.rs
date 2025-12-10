use common::{file::process_file_into_lines, impl_day};

use std::path::Path;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(5, YEAR, "day05.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let mut ranges: Vec<FreshIngredientRange> = vec![];
        let mut fresh_ingredients = 0i64;
        for line in lines {
            if line.contains("-") {
                let split: Vec<i64> = line.split('-').map(|s| s.parse().unwrap()).collect();
                ranges.push(FreshIngredientRange(split[0], split[1]));
            } else if line.len() == 0 {
                // do nothing
            } else {
                if ranges.iter()
                    .any(|range| range.get_status(line.parse().unwrap()) == Ingredient::Fresh) {
                    fresh_ingredients += 1;
                }
            }
        }


        Ok(fresh_ingredients)
    },

    puzzle2: |_day: &Day, path: &Path| {
        let lines = process_file_into_lines(path)?;

        let mut ranges: Vec<(i64, i64)> = vec![];

        for line in lines {
            if line.contains('-') {
                let split: Vec<i64> = line.split('-').map(|s| s.parse().unwrap()).collect();
                ranges.push((split[0], split[1]));
            }
        }

        // Merge overlapping ranges
        ranges.sort_unstable();
        let mut merged: Vec<(i64, i64)> = vec![];

        for (start, end) in ranges {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 + 1 {
                    // Overlapping or adjacent, merge
                    last.1 = last.1.max(end);
                } else {
                    merged.push((start, end));
                }
            } else {
                merged.push((start, end));
            }
        }

        // Count total ingredients in merged ranges
        let count: i64 = merged.iter()
            .map(|(start, end)| end - start + 1)
            .sum();

        Ok(count)
    }
});

impl Day {}

struct FreshIngredientRange(i64, i64);

impl FreshIngredientRange {
    fn get_status(&self, ingredient: i64) -> Ingredient {
        if (self.0..=self.1).contains(&ingredient) {
            Ingredient::Fresh
        } else {
            Ingredient::Spoiled
        }
    }
}

#[derive(PartialEq)]
enum Ingredient {
    Fresh,
    Spoiled,
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day05-test.txt";

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
        let expected: i64 = 14;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
