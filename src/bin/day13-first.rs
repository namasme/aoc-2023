use aoc_2023::day13;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day13/input").unwrap();
    let patterns = day13::parse_input(&input);
    let answer = summarize_patterns(&patterns);

    println!("{answer}");
}

fn summarize_patterns(patterns: &[day13::Pattern]) -> usize {
    patterns
        .iter()
        .map(day13::Pattern::find_reflection)
        .map(|reflection| reflection.summarize())
        .sum()
}
