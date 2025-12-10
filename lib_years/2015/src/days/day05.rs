use std::path::Path;

use common::{file::read_file, impl_day};
use fancy_regex::Regex;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(5, YEAR, "day05.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let file_string = read_file(path).unwrap();
        let nice_strings = Day::process_data(file_string, Day::is_string_nice);
        Ok(nice_strings)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let file_string = read_file(path).unwrap();
        let nice_strings = Day::process_data(file_string, Day::is_string_nice_p2);
        Ok(nice_strings)
    },
});

impl Day {
    fn process_data(file_string: String, cb: impl Fn(&str) -> bool) -> i32 {
        let mut nice_strings = 0;
        for line in file_string.lines() {
            if cb(&line) {
                nice_strings += 1;
            }
        }
        nice_strings
    }

    /**
      String is nice if the following:
       * at least 3 vowels
       * double letter
       * does not have: ab, cd, pq, xy
    */
    fn is_string_nice(input: &str) -> bool {
        if input.contains("ab")
            || input.contains("cd")
            || input.contains("pq")
            || input.contains("xy")
        {
            return false;
        }

        let mut last_char: Option<char> = None;
        let mut vowels: i32 = 0;
        let mut double = false;
        for c in input.chars() {
            if Some(c) == last_char {
                // double letter
                double = true;
            }
            last_char = Some(c);

            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowels += 1;
            }
        }
        if vowels >= 3 && double == true {
            return true;
        }

        false
    }
    /**
      String is nice if the following:
       * at least 3 vowels
       * double letter
       * does not have: ab, cd, pq, xy
    */
    fn is_string_nice_p2(input: &str) -> bool {
        let pair_regex = Regex::new(r"(\w{2}).*?\1").unwrap();
        let other_regex = Regex::new(r"(\w{1}).\1").unwrap();

        let pair_match = pair_regex.is_match(input).unwrap();
        let other_match = other_regex.is_match(input).unwrap();

        println!(
            "str: {}, pair: {}, other: {}",
            input, pair_match, other_match
        );

        if pair_match && other_match {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("ugknbfddgicrmopn" => true)]
    #[test_case("aaa" => true)]
    #[test_case("jchzalrnumimnmhp" => false)]
    #[test_case("haegwjzuvuyypxyu" => false)]
    #[test_case("dvszwmarrgswjxmb" => false)]
    fn test_string_nice(input: &str) -> bool {
        Day::is_string_nice(input)
    }

    #[test_case("qjhvhtzxzqqjkmpb" => true)]
    #[test_case("xxyxx" => true)]
    #[test_case("uurcxstgmygtbstg" => false)]
    #[test_case("ieodomkazucvgmuy" => false)]
    fn test_string_nice2(input: &str) -> bool {
        Day::is_string_nice_p2(input)
    }
}
