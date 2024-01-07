use aoc_2023::day6;
use std::fs;


fn main() {
    let input = fs::read_to_string("data/day6/input").unwrap();
    let (time, distance) = parse_input(&input);

    let answer = day6::beat_record(time, distance);
    println!("{:?}", answer);
}

fn parse_input(input: &str) -> (day6::Time, day6::Distance) {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let time = parse_line(time_line);
    let distance = parse_line(distance_line);

    (time, distance)
}

fn parse_line(line: &str) -> u64 {
    parse_integer_ignoring_whitespace(line.split_once(':').unwrap().1)
}

fn parse_integer_ignoring_whitespace(input: &str) -> u64 {
    input
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}
