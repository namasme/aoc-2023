use aoc_2023::day4;
use std::fs;
use std::str;

fn main() {
    let input = fs::read_to_string("data/day4/input").unwrap();
    let cards = parse_input(&input);
    let answer: u64 = cards.iter().map(day4::Scratchcard::score).sum();
    println!("{}", answer);
}

fn parse_input(input: &str) -> Vec<day4::Scratchcard> {
    input.lines().flat_map(str::parse).collect()
}
