use aoc_2023::day3;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day3/input").unwrap();
    let schematic = day3::parse_input(&input);
    let gears = find_gears(&schematic);
    let answer: u32 = gears.iter().map(calculate_gear_ratio).sum();
    println!("{}", answer);
}

type Gear = (u32, u32);

fn calculate_gear_ratio((x, y): &Gear) -> u32 {
    x * y
}

fn find_gears(schematic: &day3::Schematic) -> Vec<Gear> {
    let mut star_numbers: HashMap<day3::Position, Vec<u32>> = HashMap::new();

    for &number in schematic.numbers.iter() {
        for star_position in find_surrounding_star(schematic, number) {
            if star_numbers.contains_key(&star_position) {
                star_numbers
                    .get_mut(&star_position)
                    .unwrap()
                    .push(number.value);
            } else {
                star_numbers.insert(star_position, vec![number.value]);
            }
        }
    }

    star_numbers
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| (numbers[0], numbers[1]))
        .collect()
}

fn find_surrounding_star(schematic: &day3::Schematic, number: day3::Number) -> Vec<day3::Position> {
    day3::perimeter(number)
        .into_iter()
        .filter(|position| {
            schematic
                .symbols
                .get(position)
                .is_some_and(|&symbol| symbol == '*')
        })
        .collect()
}
