use aoc_2023::day16;
use aoc_2023::spatial;

use std::fs;

fn main() {
    let input = fs::read_to_string("data/day16/input").unwrap();
    let contraption = day16::Contraption::parse(&input);
    let answer = contraption.count_energized_tiles(day16::Beam {
        position: spatial::UPoint2D::from(0, 0),
        direction: spatial::Direction::Right,
    });

    println!("{answer}");
}
