use aoc_2023::day15;
use std::fs;

type Step = String;

fn main() {
    let input = fs::read_to_string("data/day15/input").unwrap();
    let steps = parse_input(&input);
    let answer = steps
        .iter()
        .map(|step| day15::hash(step) as u64)
        .sum::<u64>();

    println!("{answer}");
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .split(',')
        .map(|step| step.trim().to_string())
        .collect()
}
