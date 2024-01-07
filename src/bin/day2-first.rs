use aoc_2023::day2;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day2/input").unwrap();
    let games = day2::parse_input(&input);

    println!("{}", possible_games_checksum(games));
}

fn possible_games_checksum(games: Vec<day2::Game>) -> u32 {
    let mut accumulator = 0;
    let h0 = (12, 13, 14);

    for game in games {
        let estimator = day2::maxima(game.samples);
        if estimator.0 <= h0.0 && estimator.1 <= h0.1 && estimator.2 <= h0.2 {
            accumulator += game.id;
        }
    }

    accumulator
}
