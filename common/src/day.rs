use std::{error::Error, fmt::Debug, path::Path};

pub trait AdventDay {
    fn puzzle1(&self, path: &Path) -> Result<impl Debug, Box<dyn Error>> {
        let _ = path;
        Ok("test incomplete")
    }
    fn puzzle2(&self, path: &Path) -> Result<impl Debug, Box<dyn Error>> {
        let _ = path;
        Ok("test incomplete")
    }
    fn print_puzzles(&self, day: i32, path1: &Path, path2: &Path) -> String {
        format!(
            "\n\tday {} results: \n\t\tPuzzle1: {:?}\n\t\tPuzzle2: {:?}",
            day,
            self.puzzle1(path1).unwrap(),
            self.puzzle2(path2).unwrap()
        )
    }
    fn run(&self) -> String;
}

#[macro_export]
macro_rules! impl_day {
    ($day_num:expr, $year:expr, $file:expr) => {
        impl $crate::day::AdventDay for Day {
            fn run(&self) -> String {
                let data_path = $crate::file::get_data_path($year, $file);
                self.print_puzzles($day_num, data_path.as_path(), data_path.as_path())
            }
        }
    };
    ($day_num:expr, $year:expr, $file:expr, {
        puzzle1: $puzzle1_body:expr,
        puzzle2: $puzzle2_body:expr $(,)?
    }) => {
        impl $crate::day::AdventDay for Day {
            fn puzzle1(
                &self,
                path: &std::path::Path,
            ) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
                $puzzle1_body(self, path)
            }

            fn puzzle2(
                &self,
                path: &std::path::Path,
            ) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
                $puzzle2_body(self, path)
            }

            fn run(&self) -> String {
                let data_path = $crate::file::get_data_path($year, $file);
                self.print_puzzles($day_num, data_path.as_path(), data_path.as_path())
            }
        }
    };
}
