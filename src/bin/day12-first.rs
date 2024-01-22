use aoc_2023::day12;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day12/input").unwrap();
    let records = day12::parse_input(&input);
    let answer: usize = records
        .iter()
        .map(|record| record.count_arrangements())
        .sum();

    println!("{answer}");
}
