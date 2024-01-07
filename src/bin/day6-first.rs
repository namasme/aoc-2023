use aoc_2023::day6;
use std::fs;
use std::iter::zip;


fn main() {
    let input = fs::read_to_string("data/day6/input").unwrap();
    let (times, distances) = parse_input(&input);

    let answer = zip(times, distances)
        .map(|(total_time, target_distance)| day6::beat_record(total_time, target_distance))
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{:?}", answer);
}

fn parse_input(input: &str) -> (Vec<day6::Time>, Vec<day6::Distance>) {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let times = time_line
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();
    let distances = distance_line
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();

    (times, distances)
}
