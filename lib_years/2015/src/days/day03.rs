use std::path::Path;

use common::{file::read_file, impl_day};

use crate::common_values::YEAR;

pub struct Day;

impl_day!(3, YEAR, "day03.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        let result = Day::distinct_houses(&string);
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let string = read_file(path).unwrap();
        let result = Day::dos_santas_houses(&string);
        Ok(result)
    },
});

impl Day {
    fn distinct_houses(input: &str) -> i32 {
        let mut points: Vec<Point> = vec![Point::new(0, 0)];
        for c in input.chars() {
            let last = points.last().unwrap();
            match c {
                '^' => points.push(Point::new(last.x, last.y + 1)),
                '>' => points.push(Point::new(last.x + 1, last.y)),
                '<' => points.push(Point::new(last.x - 1, last.y)),
                'v' => points.push(Point::new(last.x, last.y - 1)),
                _ => {}
            };
        }
        points.sort();
        points.dedup();
        points.len() as i32
    }

    fn dos_santas_houses(input: &str) -> i32 {
        let mut santa_points: Vec<Point> = vec![Point::new(0, 0)];
        let mut robo_points: Vec<Point> = vec![Point::new(0, 0)];
        let mut is_robo = false;
        for c in input.chars() {
            let last: &Point;
            if is_robo {
                last = robo_points.last().unwrap();
            } else {
                last = santa_points.last().unwrap();
            }
            let newish_point: Option<Point> = match c {
                '^' => Some(Point::new(last.x, last.y + 1)),
                '>' => Some(Point::new(last.x + 1, last.y)),
                '<' => Some(Point::new(last.x - 1, last.y)),
                'v' => Some(Point::new(last.x, last.y - 1)),
                _ => None,
            };
            if let Some(new_point) = newish_point {
                if is_robo {
                    robo_points.push(new_point);
                } else {
                    santa_points.push(new_point);
                }
            }

            is_robo = !is_robo
        }
        let mut total_points: Vec<Point> = [santa_points, robo_points].concat();
        total_points.sort();
        total_points.dedup();
        total_points.len() as i32
    }
}

#[derive(Debug, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(">" => 2; "east1")]
    #[test_case("^>v<" => 4; "4total")]
    #[test_case("^v^v^v^v^v" => 2; "2lucky")]
    fn test_calc_wrapping_paper(input: &str) -> i32 {
        Day::distinct_houses(input)
    }

    #[test_case("^v" => 3; "3-2")]
    #[test_case("^>v<" => 3; "3-4")]
    #[test_case("^v^v^v^v^v" => 11)]
    fn test_robo_houses(input: &str) -> i32 {
        Day::dos_santas_houses(input)
    }
}
