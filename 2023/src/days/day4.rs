use std::{error::Error, fs, path::Path};

pub fn print_puzzles() {
    let path = Path::new("data/day4-input.txt");
    println!(
        "day 4 results: {}, {}",
        puzzle1(path).unwrap(),
        puzzle2(path).unwrap()
    )
}

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    processed: bool,
}

impl Card {
    fn parse(line: &str) -> Card {
        let game_and_numbers: Vec<&str> = line.split(':').map(|section| section.trim()).collect();
        // retrieve card number
        let game_number = game_and_numbers[0]
            .replace("Card", "")
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();

        let mut winning_numbers: Vec<i32> = Vec::new();
        let mut numbers: Vec<i32> = Vec::new();

        for (i, number_set) in game_and_numbers[1].split('|').enumerate() {
            let mut set: Vec<i32> = number_set
                .split(' ')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect::<Vec<i32>>();
            match i {
                0 => winning_numbers.append(&mut set),
                1 => numbers.append(&mut set),
                _ => {}
            }
        }

        Card {
            index: game_number,
            winning_numbers,
            numbers,
            processed: false,
        }
    }

    fn winners(&self) -> usize {
        let winners: Vec<i32> = self
            .numbers
            .clone()
            .into_iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect();

        winners.len()
    }

    fn points(&self) -> i32 {
        let winners: Vec<i32> = self
            .numbers
            .clone()
            .into_iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect();

        let mut result = 0;
        for (_i, _val) in winners.iter().enumerate() {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }

        result
    }
}

fn puzzle1(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;

    let cards: Vec<Card> = input_value.lines().map(Card::parse).collect();

    let mut result = 0;
    for card in cards {
        result += card.points();
    }

    Ok(result)
}

fn puzzle2(path: &Path) -> Result<i32, Box<dyn Error>> {
    let input_value = fs::read_to_string(path)?;

    let cards: Vec<Card> = input_value.lines().map(Card::parse).collect();

    let mut cards_to_process: Vec<Card> = cards.clone();
    let mut total_cards: i32 = 0;
    while !cards_to_process.is_empty() {
        let mut new_cards_to_process: Vec<Card> = Vec::new();
        for c in cards_to_process.iter_mut() {
            total_cards += 1;
            let matches = c.winners();
            for m in c.index + 1..=c.index + matches {
                let copy_card = cards.iter().find(|card| card.index == m).unwrap();
                new_cards_to_process.push(copy_card.clone());
            }
            c.processed = true;
        }
        println!(
            "cards left: {} | cards processed: {}",
            new_cards_to_process.len(),
            total_cards
        );
        cards_to_process.retain(|c| !c.processed);
        cards_to_process.append(&mut new_cards_to_process);
    }

    Ok(total_cards)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_DATA: &str = "data/day4-test-input.txt";
    #[test]
    fn test_puzzle1() {
        let result = puzzle1(Path::new(TEST_DATA));

        assert_eq!(13, result.unwrap())
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(Path::new(TEST_DATA));

        assert_eq!(30, result.unwrap())
    }
}
