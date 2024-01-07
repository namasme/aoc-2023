use aoc_2023::day2;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day2/input").unwrap();
    let games = day2::parse_input(&input);

    println!("{}", total_power(games));
}

fn total_power(games: Vec<day2::Game>) -> u32 {
    games
        .into_iter()
        .map(|game| game.samples)
        .map(day2::maxima)
        .map(power)
        .sum()
}

fn power((red, green, blue): (u32, u32, u32)) -> u32 {
    red * green * blue
}
