use std::{error::Error, fs, num::ParseIntError, path::Path};

pub fn print_puzzles() {
    let path = Path::new("data/day06-input.txt");
    println!(
        "day 6 results: {}, {}",
        puzzle1(path).unwrap(),
        puzzle2(path).unwrap()
    )
}

struct Race {
    time: i64,
    distance: i64,
}

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

    let mut winners = 0;
    for i in 0..time {
        let remaining_time = time - i;
        let distance_travelled = i * remaining_time;
        if distance_travelled > distance {
            winners += 1;
        }
    }

    Ok(winners)
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

    static TEST_DATA: &str = "data/day06-test-input.txt";
    #[test]
    fn test_puzzle1() {
        let result = puzzle1(Path::new(TEST_DATA));

        assert_eq!(288, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(Path::new(TEST_DATA));

        assert_eq!(71503, result.unwrap())
    }
}
