use std::path::Path;
use std::time::Instant;

use common::{file::read_file, impl_day};
use grid::Grid;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(6, YEAR, "day06.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let file_string = read_file(path).unwrap();
        let start = Instant::now();
        let lights_on = Day::process_data(file_string);
        println!("Day 6 Puzzle 1 took: {:?}", start.elapsed());
        Ok(lights_on)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let file_string = read_file(path).unwrap();
        let start = Instant::now();
        let total_brightness = Day::process_data_p2(file_string);
        println!("Day 6 Puzzle 2 took: {:?}", start.elapsed());
        Ok(total_brightness)
    },
});

impl Day {
    fn process_data(file_string: String) -> i32 {
        let mut grid = Grid::new(1000, 1000);
        for line in file_string.lines() {
            let cmd = Day::create_cmd(line);
            Day::process_cmd(&mut grid, cmd);
        }

        grid.iter().filter(|&&c| c).count() as i32
    }

    fn process_data_p2(file_string: String) -> i32 {
        let mut grid = Grid::new(1000, 1000);
        for line in file_string.lines() {
            let cmd = Day::create_cmd(line);
            Day::process_cmd_p2(&mut grid, cmd);
        }

        grid.iter().sum()
    }

    fn create_cmd(line: &str) -> Command {
        let split = line.split(' ').rev();
        let mut start_point: Option<Point> = None;
        let mut end_point: Option<Point> = None;
        let mut cmd_str: Option<&str> = None;
        for (i, s) in split.enumerate() {
            if i == 0 || i == 2 {
                let nums: Vec<i32> = s.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
                let point = Some(Point {
                    x: nums[0],
                    y: nums[1],
                });
                if i == 0 {
                    end_point = point;
                } else {
                    start_point = point;
                }
            }

            if i == 3 {
                cmd_str = Some(s);
            }
        }

        match cmd_str {
            Some("on") => Command::On(start_point.unwrap(), end_point.unwrap()),
            Some("off") => Command::Off(start_point.unwrap(), end_point.unwrap()),
            Some("toggle") => Command::Toggle(start_point.unwrap(), end_point.unwrap()),
            _ => panic!("command not matched"),
        }
    }

    fn process_cmd(grid: &mut Grid<bool>, cmd: Command) {
        // row
        let (rows_start, rows_end, columns_start, columns_end): (i32, i32, i32, i32) = match cmd {
            Command::On(start_point, end_point) => process_points(start_point, end_point),
            Command::Off(start_point, end_point) => process_points(start_point, end_point),
            Command::Toggle(start_point, end_point) => process_points(start_point, end_point),
        };
        for row in rows_start..=rows_end {
            for col in columns_start..=columns_end {
                let cell = grid.get_mut(row, col).unwrap();
                *cell = match cmd {
                    Command::On(_point, _point1) => true,
                    Command::Off(_point, _point1) => false,
                    Command::Toggle(_point, _point1) => !*cell,
                }
            }
        }
    }

    fn process_cmd_p2(grid: &mut Grid<i32>, cmd: Command) {
        // row
        let (rows_start, rows_end, columns_start, columns_end): (i32, i32, i32, i32) = match cmd {
            Command::On(start_point, end_point) => process_points(start_point, end_point),
            Command::Off(start_point, end_point) => process_points(start_point, end_point),
            Command::Toggle(start_point, end_point) => process_points(start_point, end_point),
        };
        for row in rows_start..=rows_end {
            for col in columns_start..=columns_end {
                let cell = grid.get_mut(row, col).unwrap();
                *cell = match cmd {
                    Command::On(_point, _point1) => *cell + 1,
                    Command::Off(_point, _point1) => {
                        if *cell == 0 {
                            0
                        } else {
                            *cell - 1
                        }
                    }
                    Command::Toggle(_point, _point1) => *cell + 2,
                }
            }
        }
    }
}

fn process_points(start_point: Point, end_point: Point) -> (i32, i32, i32, i32) {
    (start_point.x, end_point.x, start_point.y, end_point.y)
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    On(Point, Point),
    Off(Point, Point),
    Toggle(Point, Point),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("turn on 0,0 through 999,999" => Command::On(Point{x:0,y:0},Point{x:999,y:999}))]
    #[test_case("toggle 0,0 through 999,0" => Command::Toggle(Point{x:0,y:0},Point{x:999,y:0}))]
    #[test_case("turn off 499,499 through 500,500" => Command::Off(Point{x:499,y:499},Point{x:500,y:500}))]
    fn test_create_cmd(input: &str) -> Command {
        Day::create_cmd(input)
    }

    #[test]
    fn test_process_cmd_0_999() {
        let mut grid = Grid::new(1000, 1000);
        let cmd = Day::create_cmd("turn on 0,0 through 999,999");

        Day::process_cmd(&mut grid, cmd);
        for cell in grid.iter() {
            assert_eq!(true, *cell)
        }
    }
}
