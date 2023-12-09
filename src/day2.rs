struct Game {
    num: i32,
    runs: Vec<GameRun>,
}
struct GameRun {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn parse(line: &str) -> Game {
        // retrieve game number
        let num_start = line.chars().position(|c| c == ' ').unwrap();
        let colon = line.chars().position(|c| c == ':').unwrap();
        let number = &line[num_start + 1..colon];
        todo!()
    }
}
