mod days;

use common::day::AdventDay;
use days::*;

fn main() {
    let day = day1::Day1;
    day.print_puzzles(1, "data/day1.txt", "data/day1.txt");
}
