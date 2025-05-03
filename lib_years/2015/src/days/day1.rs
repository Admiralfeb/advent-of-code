use common::{day::AdventDay, file::read_file};

use crate::common_values::get_data_path;

pub struct Day1;
impl AdventDay for Day1 {
    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day1.txt";
        self.print_puzzles(1, day1_path.as_str(), day1_path.as_str())
    }

    fn puzzle1(&self, path: &str) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let string = read_file(path).unwrap();

        let result = Day1::eval_elevator(&string, None);

        Ok(result)
    }

    fn puzzle2(&self, path: &str) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let string = read_file(path).unwrap();

        let result = Day1::find_basement_position(&string, None);

        Ok(result)
    }

    fn print_puzzles(&self, day: i32, path1: &str, path2: &str) -> String {
        std::format!(
            "\n\tday {} results: \n\t\tPuzzle1: {:?}\n\t\tPuzzle2: {:?}",
            day,
            self.puzzle1(path1).unwrap(),
            self.puzzle2(path2).unwrap()
        )
    }
}

impl Day1 {
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
        let result = Day1::eval_elevator(input, None);

        result
    }

    #[test_case(")" => Some(1))]
    #[test_case("()())" => Some(5))]
    #[test_case("(((" => None)]
    fn when_basement_resolves(input: &str) -> Option<i64> {
        let result = Day1::find_basement_position(input, None);

        result
    }
}
