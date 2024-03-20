use aoc_2023::day14;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day14/input").unwrap();
    let platform = day14::parse_input(&input);
    let answer = platform.run_for(4 * 1000000000).load();

    println!("{answer}");
}
