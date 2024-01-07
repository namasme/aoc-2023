use aoc_2023::day5;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day5/input").unwrap();
    let (seeds, maps) = parse_input(&input);
    let answer = seeds
        .iter()
        .map(|&seed| apply_all(seed, &maps))
        .min()
        .unwrap();

    println!("{}", answer);
}

fn apply_all(seed: day5::Identifier, maps: &Vec<day5::Map>) -> day5::Identifier {
    maps.iter().fold(seed, |seed, map| map.apply(seed))
}

fn parse_input(input: &str) -> (Vec<day5::Identifier>, Vec<day5::Map>) {
    let (seeds_block, rest) = input.split_once("\n\n").unwrap();
    let seeds = seeds_block
        .split(' ')
        .skip(1)
        .flat_map(str::parse)
        .collect();
    let maps = rest.split("\n\n").flat_map(str::parse).collect();

    (seeds, maps)
}
