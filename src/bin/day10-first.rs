use aoc_2023::day10;
use aoc_2023::spatial::UPoint2D;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day10/input").unwrap();
    let (field, start) = day10::parse_input(&input);
    let answer = find_length(&field, start);
    println!("{answer}");
}

fn find_length(field: &day10::Field, start: UPoint2D) -> u64 {
    let (direction, _) = field.identify_start_pipe(start);
    let steps = field.trace_loop(start, direction);

    (steps.len() as u64 + 1) / 2
}
