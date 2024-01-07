use std::cmp;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(raw_color: &str) -> Self {
        match raw_color {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("error parsing color"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub samples: Vec<Vec<(u32, Color)>>,
}

pub fn maxima(samples: Vec<Vec<(u32, Color)>>) -> (u32, u32, u32) {
    let mut accumulator = HashMap::new();

    for sample in samples {
        for (count, color) in sample {
            let current = *accumulator.get(&color).unwrap_or(&0);
            accumulator.insert(color, cmp::max(current, count));
        }
    }

    (
        accumulator[&Color::Red],
        accumulator[&Color::Green],
        accumulator[&Color::Blue],
    )
}

pub fn parse_input(raw_input: &str) -> Vec<Game> {
    raw_input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Game {
    let (prefix, rest) = line.split_once(": ").unwrap();
    let id: u32 = prefix.split_once(' ').unwrap().1.parse().unwrap();
    let samples = rest.split("; ").map(parse_sample).collect();

    Game { id, samples }
}

fn parse_sample(raw_sample: &str) -> Vec<(u32, Color)> {
    raw_sample
        .split(", ")
        .map(|entry| {
            let (raw_count, raw_color) = entry.split_once(' ').unwrap();
            let count: u32 = raw_count.parse().unwrap();
            let color: Color = Color::parse(raw_color);

            (count, color)
        })
        .collect()
}
