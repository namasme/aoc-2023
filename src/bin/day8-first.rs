use aoc_2023::day8;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day8/input").unwrap();
    let (directions, network) = day8::parse_input(&input);
    let answer = distance(
        String::from("AAA"),
        String::from("ZZZ"),
        directions,
        network,
    );
    println!("{}", answer);
}

fn distance(
    from: String,
    to: String,
    directions: Vec<day8::Direction>,
    network: day8::Network,
) -> u32 {
    let mut cycled = directions.iter().cycle();
    let mut current = from;
    let mut steps = 0;

    while current != to {
        steps += 1;
        let direction = cycled.next().unwrap();
        let (left, right) = &network.edges[&current];
        current = match direction {
            day8::Direction::Left => left.to_string(),
            day8::Direction::Right => right.to_string(),
        };
    }

    steps
}
