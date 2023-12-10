use std::{
    error::Error,
    fs::{self},
    path::Path,
};

use regex::Regex;

pub fn print_puzzles() {
    let path = Path::new("data/day1-input.txt");
    println!(
        "day 1 results: {}, {}",
        puzzle1(path).unwrap(),
        puzzle2(path).unwrap()
    )
}

/*
* Iterate through the digits in each line and create a number. Sum the results of all line numbers.
*/
fn puzzle1(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;
    let mut result: i32 = 0;
    for line in input_value.lines() {
        let regex = Regex::new(r"[0-9]").unwrap();
        let digits: Vec<String> = regex
            .find_iter(line)
            .filter_map(|digit| digit.as_str().parse::<i32>().ok())
            .map(|digit| digit.to_string())
            .collect();
        let r = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let number = r.parse::<i32>().ok().unwrap();
        result += number;
    }
    Ok(result)
}

fn puzzle2(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;
    let mut result: i32 = 0;
    for line in input_value.lines() {
        let regex =
            Regex::new(r"^([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let digs: Vec<String> = line
            .char_indices()
            .filter_map(|(i, _)| regex.captures(&line[i..]))
            .filter_map(|s| match s[0].to_string().as_str() {
                "0" | "zero" => Some("0"),
                "1" | "one" => Some("1"),
                "2" | "two" => Some("2"),
                "3" | "three" => Some("3"),
                "4" | "four" => Some("4"),
                "5" | "five" => Some("5"),
                "6" | "six" => Some("6"),
                "7" | "seven" => Some("7"),
                "8" | "eight" => Some("8"),
                "9" | "nine" => Some("9"),
                _ => None,
            })
            .map(|v| v.to_string())
            .collect();

        // let digs: Vec<String> = line
        //     .char_indices()
        //     .filter_map(|(i, _)| {
        //         regex
        //             .find_iter(&line[i..])
        //             .filter_map(|digit| {
        //                 return match digit.as_str() {
        //                     "0" | "zero" => Some("0"),
        //                     "1" | "one" => Some("1"),
        //                     "2" | "two" => Some("2"),
        //                     "3" | "three" => Some("3"),
        //                     "4" | "four" => Some("4"),
        //                     "5" | "five" => Some("5"),
        //                     "6" | "six" => Some("6"),
        //                     "7" | "seven" => Some("7"),
        //                     "8" | "eight" => Some("8"),
        //                     "9" | "nine" => Some("9"),
        //                     _ => None,
        //                 };
        //             })
        //             .map(|digit| Some(digit.to_string()))
        //             .collect::<Option<String>>()
        //     })
        //     .collect();
        // let digits: Vec<String> = regex
        //     .find_iter(line)
        //     .filter_map(|digit| {
        //         return match digit.as_str() {
        //             "0" | "zero" => Some("0"),
        //             "1" | "one" => Some("1"),
        //             "2" | "two" => Some("2"),
        //             "3" | "three" => Some("3"),
        //             "4" | "four" => Some("4"),
        //             "5" | "five" => Some("5"),
        //             "6" | "six" => Some("6"),
        //             "7" | "seven" => Some("7"),
        //             "8" | "eight" => Some("8"),
        //             "9" | "nine" => Some("9"),
        //             _ => None,
        //         };
        //     })
        //     .map(|digit| digit.to_string())
        //     .collect();
        let r = format!("{}{}", digs.first().unwrap(), digs.last().unwrap());
        let number = r.parse::<i32>().ok().unwrap();
        result += number;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = Path::new("data/day1-test-input.txt");
        let result = puzzle1(path).unwrap();
        assert_eq!(142, result);
    }

    #[test]
    fn test_puzzle2() {
        let path = Path::new("data/day1-test-input2.txt");
        let result = puzzle2(path).unwrap();
        assert_eq!(281, result);
    }
}
