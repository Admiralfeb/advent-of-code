use std::{error::Error, fmt::Debug};

pub trait AdventDay {
    fn puzzle1(&self, path: &str) -> Result<impl Debug, Box<dyn Error>> {
        let _ = path;
        Ok("test incomplete")
    }
    fn puzzle2(&self, path: &str) -> Result<impl Debug, Box<dyn Error>> {
        let _ = path;
        Ok("test incomplete")
    }
    fn print_puzzles(&self, day: i32, path1: &str, path2: &str) -> () {
        println!(
            "day {} results: {:?}, {:?}",
            day,
            self.puzzle1(path1).unwrap(),
            self.puzzle2(path2).unwrap()
        )
    }
    fn run(&self);
}
