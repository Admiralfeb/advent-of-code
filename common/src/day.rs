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
