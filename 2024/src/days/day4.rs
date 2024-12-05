use std::{collections::HashSet, error::Error};

use common::{day::AdventDay, file::read_file};
use grid::*;

use crate::common_values::get_data_path;

pub struct Day4;
impl AdventDay for Day4 {
    // find xmas
    fn puzzle1(&self, path: &str) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
        let binding = read_file(path)?;
        let mut number_found: i64 = 0;

        let lines = binding.lines();

        let mut grid = grid![];
        for line in lines {
            let line_chars: Vec<char> = line.chars().collect();
            grid.push_row(line_chars);
        }

        let right_bound = (grid.cols()) as i64;
        let bottom_bound = (grid.rows()) as i64;

        for r in 0..bottom_bound {
            for c in 0..right_bound {
                number_found += check_for_word(r, c, &grid);
            }
        }
        Ok(number_found)
    }
    fn puzzle2(&self, path: &str) -> Result<impl std::fmt::Debug, Box<dyn Error>> {
        let binding = read_file(path)?;

        let lines = binding.lines();

        let mut grid = grid![];
        for line in lines {
            let line_chars: Vec<char> = line.chars().collect();
            grid.push_row(line_chars);
        }

        let right_bound = (grid.cols()) as i64;
        let bottom_bound = (grid.rows()) as i64;

        let mut mas_matches = Vec::new();

        for r in 0..bottom_bound {
            for c in 0..right_bound {
                mas_matches = [mas_matches, check_for_mas(r, c, &grid)].concat();
            }
        }
        println!("mas matches: {}", mas_matches.len());

        // filter out distinct A coords
        let mut all_a_coords: Vec<LetterCoordinate> =
            mas_matches.iter().map(|m| m.a.clone()).collect();
        println!("coords: {}", all_a_coords.len());
        let mut deduped = HashSet::new();
        all_a_coords.retain(|e| !deduped.insert(e.clone()));

        Ok(all_a_coords.len())

        // Ok(number_found)
    }

    fn run(&self) -> String {
        let data_path = get_data_path().to_string();
        let day1_path = data_path + "day4.txt";
        self.print_puzzles(3, day1_path.as_str(), day1_path.as_str())
    }
}

fn check_for_word(row: i64, column: i64, grid: &Grid<char>) -> i64 {
    let mut times_found: i64 = 0;
    let letter = grid.get(row, column).unwrap();
    if *letter == 'X' {
        let x_coor = LetterCoordinate { row, col: column };
        // check north
        if check_for_letter(&x_coor, WordDirection::North, &grid) {
            times_found += 1;
        }

        // check ne
        if check_for_letter(&x_coor, WordDirection::NorthEast, &grid) {
            times_found += 1;
        }

        // check east
        if check_for_letter(&x_coor, WordDirection::East, &grid) {
            times_found += 1;
        }

        // check se
        if check_for_letter(&x_coor, WordDirection::SouthEast, &grid) {
            times_found += 1;
        }

        // check south
        if check_for_letter(&x_coor, WordDirection::South, &grid) {
            times_found += 1;
        }

        // check sw
        if check_for_letter(&x_coor, WordDirection::SouthWest, &grid) {
            times_found += 1;
        }

        // check west
        if check_for_letter(&x_coor, WordDirection::West, &grid) {
            times_found += 1;
        }

        // check nw
        if check_for_letter(&x_coor, WordDirection::NorthWest, &grid) {
            times_found += 1;
        }
    }

    times_found
}

fn check_for_letter(
    x_coor: &LetterCoordinate,
    direction: WordDirection,
    grid: &Grid<char>,
) -> bool {
    let col = x_coor.col;
    match direction {
        WordDirection::North => {
            let m_coor = LetterCoordinate {
                row: x_coor.row - 1,
                col,
            };
            let a_coor = LetterCoordinate {
                row: m_coor.row - 1,
                col,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row - 1,
                col,
            };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::NorthEast => {
            let m_coor = LetterCoordinate {
                row: x_coor.row - 1,
                col: col + 1,
            };
            let a_coor = LetterCoordinate {
                row: m_coor.row - 1,
                col: col + 2,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row - 1,
                col: col + 3,
            };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::East => {
            let row = x_coor.row;
            let m_coor = LetterCoordinate { row, col: col + 1 };
            let a_coor = LetterCoordinate { row, col: col + 2 };
            let s_coor = LetterCoordinate { row, col: col + 3 };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::SouthEast => {
            let m_coor = LetterCoordinate {
                row: x_coor.row + 1,
                col: col + 1,
            };
            let a_coor = LetterCoordinate {
                row: m_coor.row + 1,
                col: col + 2,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row + 1,
                col: col + 3,
            };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::South => {
            let m_coor = LetterCoordinate {
                row: x_coor.row + 1,
                col,
            };
            let a_coor = LetterCoordinate {
                row: m_coor.row + 1,
                col,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row + 1,
                col,
            };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::SouthWest => {
            let m_coor = LetterCoordinate {
                row: x_coor.row + 1,
                col: col - 1,
            };
            let a_coor = LetterCoordinate {
                row: m_coor.row + 1,
                col: col - 2,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row + 1,
                col: col - 3,
            };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::West => {
            let row = x_coor.row;
            let m_coor = LetterCoordinate { row, col: col - 1 };
            let a_coor = LetterCoordinate { row, col: col - 2 };
            let s_coor = LetterCoordinate { row, col: col - 3 };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
        WordDirection::NorthWest => {
            let m_coor = LetterCoordinate {
                row: x_coor.row - 1,
                col: col - 1,
            };
            let a_coor = LetterCoordinate {
                row: m_coor.row - 1,
                col: col - 2,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row - 1,
                col: col - 3,
            };

            word_matches(grid, direction, x_coor.clone(), m_coor, a_coor, s_coor)
        }
    }
}

fn word_matches(
    grid: &Grid<char>,
    _direction: WordDirection,
    _x_coor: LetterCoordinate,
    m_coor: LetterCoordinate,
    a_coor: LetterCoordinate,
    s_coor: LetterCoordinate,
) -> bool {
    let m_option = grid.get(m_coor.row, m_coor.col);
    let a_option = grid.get(a_coor.row, a_coor.col);
    let s_option = grid.get(s_coor.row, s_coor.col);

    if m_option.is_some_and(|x| *x == 'M')
        && a_option.is_some_and(|x| *x == 'A')
        && s_option.is_some_and(|x| *x == 'S')
    {
        // let word = WordCoordinate {
        //     x: x_coor,
        //     m: m_coor,
        //     a: a_coor,
        //     s: s_coor,
        // };
        // println!(
        //     "word match found: \n\tdirection: {:?} \n\tlocation: {:?}",
        //     direction, word
        // );
        return true;
    }
    false
}

fn check_for_mas(row: i64, column: i64, grid: &Grid<char>) -> Vec<MasCoordinate> {
    let letter = grid.get(row, column).unwrap();
    let mut mas_matches: Vec<MasCoordinate> = Vec::new();
    if *letter == 'M' {
        let m_coor = LetterCoordinate { row, col: column };

        // check ne
        mas_matches = [
            mas_matches,
            check_for_letter2(&m_coor, MasDirection::NorthEast, &grid),
        ]
        .concat();

        // check se

        mas_matches = [
            mas_matches,
            check_for_letter2(&m_coor, MasDirection::SouthEast, &grid),
        ]
        .concat();

        // check sw
        mas_matches = [
            mas_matches,
            check_for_letter2(&m_coor, MasDirection::SouthWest, &grid),
        ]
        .concat();

        // check nw
        mas_matches = [
            mas_matches,
            check_for_letter2(&m_coor, MasDirection::NorthWest, &grid),
        ]
        .concat();
    }

    mas_matches
}

fn check_for_letter2(
    m_coor: &LetterCoordinate,
    direction: MasDirection,
    grid: &Grid<char>,
) -> Vec<MasCoordinate> {
    let col = m_coor.col;
    let mut mas_matches: Vec<MasCoordinate> = Vec::new();
    match direction {
        MasDirection::NorthEast => {
            let a_coor = LetterCoordinate {
                row: m_coor.row - 1,
                col: col + 1,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row - 1,
                col: col + 2,
            };

            if let Some(m) = word_matches2(grid, direction, m_coor.clone(), a_coor, s_coor) {
                mas_matches.push(m);
            }
        }
        MasDirection::SouthEast => {
            let a_coor = LetterCoordinate {
                row: m_coor.row + 1,
                col: col + 1,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row + 1,
                col: col + 2,
            };

            if let Some(m) = word_matches2(grid, direction, m_coor.clone(), a_coor, s_coor) {
                mas_matches.push(m);
            }
        }
        MasDirection::SouthWest => {
            let a_coor = LetterCoordinate {
                row: m_coor.row + 1,
                col: col - 1,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row + 1,
                col: col - 2,
            };

            if let Some(m) = word_matches2(grid, direction, m_coor.clone(), a_coor, s_coor) {
                mas_matches.push(m);
            }
        }
        MasDirection::NorthWest => {
            let a_coor = LetterCoordinate {
                row: m_coor.row - 1,
                col: col - 1,
            };
            let s_coor = LetterCoordinate {
                row: a_coor.row - 1,
                col: col - 2,
            };

            if let Some(m) = word_matches2(grid, direction, m_coor.clone(), a_coor, s_coor) {
                mas_matches.push(m);
            }
        }
    };

    mas_matches
}

fn word_matches2(
    grid: &Grid<char>,
    _direction: MasDirection,
    m_coor: LetterCoordinate,
    a_coor: LetterCoordinate,
    s_coor: LetterCoordinate,
) -> Option<MasCoordinate> {
    let a_option = grid.get(a_coor.row, a_coor.col);
    let s_option = grid.get(s_coor.row, s_coor.col);

    if a_option.is_some_and(|x| *x == 'A') && s_option.is_some_and(|x| *x == 'S') {
        let mas = MasCoordinate {
            m: m_coor,
            a: a_coor,
            s: s_coor,
        };
        // println!(
        //     "word match found: \n\tdirection: {:?} \n\tlocation: {:?}",
        //     direction, word
        // );
        return Some(mas);
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LetterCoordinate {
    row: i64,
    col: i64,
}

#[derive(Debug)]
struct WordCoordinate {
    x: LetterCoordinate,
    m: LetterCoordinate,
    a: LetterCoordinate,
    s: LetterCoordinate,
}
#[derive(Debug, Clone)]
struct MasCoordinate {
    m: LetterCoordinate,
    a: LetterCoordinate,
    s: LetterCoordinate,
}

#[derive(Debug)]
enum WordDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
#[derive(Debug)]
enum MasDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_puzzle1() {
        let path = "data/day4-test.txt";
        let day = Day4;
        let result = day.puzzle1(path).unwrap();
        let expected: i64 = 18;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }

    // #[test]
    fn test_puzzle2() {
        let path = "data/day4-test.txt";
        let day = Day4;
        let result = day.puzzle2(path).unwrap();
        let expected: i64 = 9;
        assert_eq!(expected.to_string(), format!("{:?}", result))
    }
}
