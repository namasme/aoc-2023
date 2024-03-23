use aoc_2023::day16;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day16/input").unwrap();
    let contraption = day16::Contraption::parse(&input);
    let answer = contraption.maximize_energized_tiles();

    println!("{answer}");
}
