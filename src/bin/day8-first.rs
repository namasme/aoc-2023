use aoc_2023::day8;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day8/input").unwrap();
    let (directions, network) = day8::parse_input(&input);
    let answer = distance(
        &"AAA".to_string(),
        &"ZZZ".to_string(),
        &directions,
        &network,
    );
    println!("{}", answer);
}

fn distance(
    from: &day8::NodeID,
    to: &day8::NodeID,
    directions: &[day8::Direction],
    network: &day8::Network,
) -> usize {
    let step = day8::Step {
        node_id: from,
        network,
        direction_stream: day8::DirectionStream::from_directions(directions),
    };
    1 + step
        .into_iter()
        .take_while(|current| current.node_id != to)
        .count()
}
