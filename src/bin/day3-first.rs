use aoc_2023::day3;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day3/input").unwrap();
    let schematic = day3::parse_input(&input);

    println!("{}", sum_part_numbers(&schematic));
}

fn sum_part_numbers(schematic: &day3::Schematic) -> u32 {
    schematic
        .numbers
        .iter()
        .filter(|&number| is_part_number(schematic, *number))
        .map(|&number| number.value)
        .sum()
}

fn is_part_number(schematic: &day3::Schematic, number: day3::Number) -> bool {
    day3::perimeter(number)
        .iter()
        .any(|position| schematic.symbols.contains_key(position))
}
