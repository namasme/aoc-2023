use aoc_2023::day14;
use aoc_2023::spatial::Direction;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day14/input").unwrap();
    let mut platform = day14::parse_input(&input);
    platform.tilt(Direction::Up);
    let answer = platform.load();

    println!("{answer}");
}
