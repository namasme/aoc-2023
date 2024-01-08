use aoc_2023::day9;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day9/input").unwrap();
    let mut report = day9::parse_input(&input);
    let answer: day9::Value = report.iter_mut().map(day9::History::predict_next).sum();
    println!("{}", answer);
}
