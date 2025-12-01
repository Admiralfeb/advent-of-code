use std::{fs, path::Path};

use common::impl_day;

use crate::common_values::YEAR;

pub struct Day;

impl_day!(2, YEAR, "day2-input.txt", {
    puzzle1: |_day: &Day, path: &Path| {
        let given: GameRun = GameRun {
            red: 12,
            green: 13,
            blue: 14,
        };
        let input_value = fs::read_to_string(path)?;
        let games: Vec<Game> = input_value.lines().map(Game::parse).collect();
        let possible_games: Vec<i32> = games
            .iter()
            .filter(|game| game.possible(&given))
            .map(|game| game.num)
            .collect();

        let mut result = 0;
        for game_num in possible_games {
            result += game_num;
        }
        Ok(result)
    },
    puzzle2: |_day: &Day, path: &Path| {
        let input_value = fs::read_to_string(path)?;
        let games: Vec<Game> = input_value.lines().map(Game::parse).collect();

        let mut result = 0;
        for game in &games {
            let power = game.power();
            result += power;
        }

        Ok(result)
    },
});

struct Game {
    num: i32,
    runs: Vec<GameRun>,
}
#[derive(Debug)]
struct GameRun {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn parse(line: &str) -> Game {
        let game_and_colors: Vec<&str> = line.split(':').map(|section| section.trim()).collect();
        // retrieve game number
        let game_number = game_and_colors[0]
            .replace("Game ", "")
            .parse::<i32>()
            .unwrap();

        // retrieve game runs
        let game_runs: Vec<GameRun> = game_and_colors[1]
            .split(';')
            .map(|text_run| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for color in text_run.split(',').map(|t| t.trim()) {
                    let values: Vec<&str> = color.split(' ').map(|t| t.trim()).collect();
                    match values[1].to_lowercase().as_str() {
                        "red" => {
                            red = values[0].parse().unwrap();
                        }
                        "green" => green = values[0].parse().unwrap(),
                        "blue" => blue = values[0].parse().unwrap(),
                        _ => {}
                    }
                }

                GameRun { red, green, blue }
            })
            .collect();

        Game {
            num: game_number,
            runs: game_runs,
        }
    }

    fn possible(&self, given: &GameRun) -> bool {
        self.runs
            .iter()
            .all(|run| run.red <= given.red && run.green <= given.green && run.blue <= given.blue)
    }

    fn power(&self) -> i32 {
        // get the fewest cubes, then multiply the resultant values
        let mut red: Option<i32> = None;
        let mut green: Option<i32> = None;
        let mut blue: Option<i32> = None;

        for run in &self.runs {
            if run.red > 0 {
                red = match red {
                    None => Some(run.red),
                    Some(x) if x < run.red => Some(run.red),
                    _ => red,
                };
            }
            if run.green > 0 {
                green = match green {
                    None => Some(run.green),
                    Some(x) if x < run.green => Some(run.green),
                    _ => green,
                };
            }
            if run.blue > 0 {
                blue = match blue {
                    None => Some(run.blue),
                    Some(x) if x < run.blue => Some(run.blue),
                    _ => blue,
                };
            }
        }

        let lowest_red = red.unwrap();
        let lowest_green = green.unwrap();
        let lowest_blue = blue.unwrap();

        lowest_red * lowest_green * lowest_blue
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{
        day::AdventDay,
        file::get_data_path,
    };

    #[test]
    fn test_puzzle1() {
        let path = get_data_path(YEAR, "day2-test-input.txt");
        let day = Day;
        let result = day.puzzle1(&path).unwrap();
        let expected: i32 = 8;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }

    #[test]
    fn test_puzzle2() {
        let path = get_data_path(YEAR, "day2-test-input.txt");
        let day = Day;
        let result = day.puzzle2(&path).unwrap();
        let expected: i32 = 2286;
        assert_eq!(expected.to_string(), format!("{:?}", result));
    }
}
