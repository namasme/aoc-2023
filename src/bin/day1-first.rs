use std::fs;

fn main() {
    let input = fs::read_to_string("data/day1/input").unwrap();
    let answer: u32 = input.lines().map(extract_calibration_value).sum();
    println!("{}", answer);
}

fn extract_calibration_value(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|char| char.is_numeric())
        .collect::<Vec<_>>();
    let first = digits[0].to_digit(10).unwrap();
    let last = digits[digits.len() - 1].to_digit(10).unwrap();

    return 10 * first + last;
}
