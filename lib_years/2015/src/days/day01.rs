use std::path::Path;

use common::{file::read_file, impl_day};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(1, YEAR, "day01.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        let result = Day::eval_elevator(&string, None);
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        let result = Day::find_basement_position(&string, None);
        Ok(result)
    },
});

impl Day {
    fn eval_elevator(input: &str, starting_floor: Option<i64>) -> i64 {
        let mut floor = starting_floor.unwrap_or(0);
        let mut increases = 0;
        let mut decreases = 0;
        for c in input.chars() {
            if c == '(' {
                floor += 1;
                increases += 1;
            } else if c == ')' {
                floor -= 1;
                decreases += 1;
            }
        }

        let diff = increases - decreases;
        if floor != diff {
            panic!(
                "floor and diff are different. Floor: {}, diff: {}",
                floor, diff
            );
        }

        floor
    }

    fn find_basement_position(input: &str, starting_floor: Option<i64>) -> Option<i64> {
        let mut floor = starting_floor.unwrap_or(0);
        for (i, c) in input.chars().enumerate() {
            if c == '(' {
                floor += 1;
            } else if c == ')' {
                floor -= 1;
            }

            if floor == -1 {
                return Some((i + 1) as i64);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("()()" => 0 ; "0 1")]
    #[test_case("(())" => 0 ; "0 2")]
    #[test_case("(((" => 3 ; "3 1")]
    #[test_case("(()(()(" => 3 ; "3 2")]
    #[test_case("))(((((" => 3 ; "3 3")]
    #[test_case("())" => -1 ; "neg1 1")]
    #[test_case("))(" => -1 ; "neg1 2")]
    #[test_case(")))" => -3 ; "neg3 1")]
    #[test_case(")())())" => -3 ; "neg3 2")]
    fn eval_elevator_should_calc_right(input: &str) -> i64 {
        let result = Day::eval_elevator(input, None);

        result
    }

    #[test_case(")" => Some(1))]
    #[test_case("()())" => Some(5))]
    #[test_case("(((" => None)]
    fn when_basement_resolves(input: &str) -> Option<i64> {
        let result = Day::find_basement_position(input, None);

        result
    }
}
