use common::{file::read_file, impl_day};
use fancy_regex::Regex;
use rayon::prelude::*;
use std::{error::Error, path::Path, sync::LazyLock};

use crate::common_values::YEAR;

static REPEAT_1_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\1$").unwrap());
static REPEAT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\1+$").unwrap());

pub struct Day;

impl_day!(2, YEAR, "day2.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        puzzle(path, &*REPEAT_1_REGEX)

    },

    puzzle2: |_day: &Day, path: &Path| {
        puzzle(path, &*REPEAT_REGEX)
    }
});

fn puzzle(path: &Path, regex: &Regex) -> Result<i64, Box<dyn Error>> {
    println!("data file path: {:?}", path);

    let file_string = read_file(path).unwrap();
    let ranges: Vec<Range> = file_string
        .trim()
        .split(',')
        .map(|string| {
            let split = string
                .split('-')
                .map(|split| {
                    split
                        .parse::<i64>()
                        .expect(&format!("Failed to parse value: {:?}", split))
                })
                .collect::<Vec<i64>>();
            Range {
                start: split[0],
                end: split[1],
            }
        })
        .collect();
    let result = Day::search_for_dupes(ranges, regex);

    Ok(result)
}

impl Day {
    fn search_for_dupes(ranges: Vec<Range>, regex: &Regex) -> i64 {
        ranges
            .par_iter()
            .map(|range| {
                (range.start..=range.end)
                    .into_par_iter()
                    .filter_map(|v| {
                        let v_string = v.to_string();
                        if regex.is_match(&v_string).unwrap_or(false) {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>()
            })
            .sum()
    }
}

struct Range {
    start: i64,
    end: i64,
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{day::AdventDay, file::get_data_path};

    const TEST_DATA_PATH: &str = "day2-test.txt";

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, TEST_DATA_PATH);
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i64 = 1227775554;
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
