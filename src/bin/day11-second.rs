use aoc_2023::day11;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day11/input").unwrap();
    let image = day11::parse_input(&input);
    let answer = image.total_distance(1000000);
    println!("{answer:?}");
}
