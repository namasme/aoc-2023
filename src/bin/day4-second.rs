use std::fs;
use std::str;
use aoc_2023::day4;

fn main() {
    let input = fs::read_to_string("data/day4/input").unwrap();
    let cards = parse_input(&input);
    let answer: u64 = count_copies(&cards);
    println!("{}", answer);
}

fn count_copies(cards: &Vec<day4::Scratchcard>) -> u64 {
    let mut counts = vec![1; cards.len()];

    for idx in 0..cards.len() {
        let current_count = counts[idx];
        let matches_count = cards[idx].count_matches();

        for delta_idx in 0..matches_count {
            counts.get_mut(idx + delta_idx + 1).map(|count| *count += current_count);
        }
    }

    counts.iter().sum()
}

fn parse_input(input: &str) -> Vec<day4::Scratchcard> {
    input.lines().flat_map(str::parse).collect()
}
