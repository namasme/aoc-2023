use std::fs;

const DIGIT_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("data/day1/input").unwrap();
    let answer: u32 = input.lines().map(extract_calibration_value).sum();

    println!("{}", answer);
}

fn extract_calibration_value(line: &str) -> u32 {
    let first = line
        .char_indices()
        .find_map(|(idx, char)| {
            char.to_digit(10).or(DIGIT_NAMES
                .iter()
                .find(|&candidate| {
                    idx + candidate.len() <= line.len()
                        && line[idx..idx + candidate.len()].starts_with(candidate)
                })
                .and_then(|&solution| parse_extended_digit(solution)))
        })
        .unwrap();

    let last = line
        .char_indices()
        .rev()
        .find_map(|(idx, char)| {
            char.to_digit(10).or(DIGIT_NAMES
                .iter()
                .find(|&candidate| {
                    idx + 1 >= candidate.len()
                        && line[idx + 1 - candidate.len()..idx + 1].starts_with(candidate)
                })
                .and_then(|&solution| parse_extended_digit(solution)))
        })
        .unwrap_or(first);

    10 * first + last
}

fn parse_extended_digit(digit: &str) -> Option<u32> {
    match digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
