use crate::spatial;
use anyhow::anyhow;
use anyhow::Result;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    pub pipes: HashMap<spatial::UPoint2D, Pipe>,
}

pub type Pipe = (spatial::Direction, spatial::Direction);

#[derive(Debug, Eq, PartialEq)]
enum RawPipe {
    Horizontal,
    Vertical,
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

impl Field {
    pub fn trace_loop(
        &self,
        start: spatial::UPoint2D,
        direction: spatial::Direction,
    ) -> Vec<spatial::UPoint2D> {
        let mut current = start.move_by(direction).unwrap();
        let mut came_from = -direction;
        let mut loop_points = vec![];

        while current != start {
            loop_points.push(current);
            let directions = self.pipes.get(&current).unwrap();
            if directions.0 == came_from {
                current = current.move_by(directions.1).unwrap();
                came_from = -directions.1;
            } else {
                current = current.move_by(directions.0).unwrap();
                came_from = -directions.0;
            }
        }
        loop_points.push(start);

        loop_points
    }

    pub fn identify_start_pipe(&self, start: spatial::UPoint2D) -> Pipe {
        let connected: Vec<_> = start
            .neighbours()
            .iter()
            .filter_map(|point| self.pipes.get(point).map(|pipe| (point, pipe)))
            .filter_map(|(point, pipe)| {
                if point.move_by(pipe.0) == Some(start) {
                    Some(-pipe.0)
                } else if point.move_by(pipe.1) == Some(start) {
                    Some(-pipe.1)
                } else {
                    None
                }
            })
            .collect();

        // The resulting Vec should contain exactly two elements for the given input.
        (
            cmp::min(connected[0], connected[1]), // for the sake of consistency, return them ordered
            cmp::max(connected[0], connected[1]),
        )
    }
}

impl RawPipe {
    fn parse(input: char) -> Result<RawPipe> {
        match input {
            '-' => Ok(RawPipe::Horizontal),
            '|' => Ok(RawPipe::Vertical),
            'J' => Ok(RawPipe::LeftTop),
            'L' => Ok(RawPipe::RightTop),
            '7' => Ok(RawPipe::LeftBottom),
            'F' => Ok(RawPipe::RightBottom),
            _ => Err(anyhow!("Invalid pipe: {}", input)),
        }
    }

    fn directions(&self) -> (spatial::Direction, spatial::Direction) {
        match self {
            RawPipe::Horizontal => (spatial::Direction::Left, spatial::Direction::Right),
            RawPipe::Vertical => (spatial::Direction::Up, spatial::Direction::Down),
            RawPipe::LeftTop => (spatial::Direction::Up, spatial::Direction::Left),
            RawPipe::RightTop => (spatial::Direction::Up, spatial::Direction::Right),
            RawPipe::LeftBottom => (spatial::Direction::Down, spatial::Direction::Left),
            RawPipe::RightBottom => (spatial::Direction::Down, spatial::Direction::Right),
        }
    }
}

pub fn parse_input(input: &str) -> (Field, spatial::UPoint2D) {
    let mut pipes = vec![];
    let mut start = None;

    for (row, o_start) in input.lines().enumerate().map(parse_line) {
        pipes.extend(row.into_iter());
        start = start.or(o_start);
    }
    (
        Field {
            pipes: pipes.into_iter().collect(),
        },
        start.unwrap(),
    )
}

fn parse_line(
    (row, line): (usize, &str),
) -> (Vec<(spatial::UPoint2D, Pipe)>, Option<spatial::UPoint2D>) {
    let mut pipes = vec![];
    let mut start = None;
    for (column, ch) in line.chars().enumerate() {
        if ch == 'S' {
            start = Some(spatial::UPoint2D::from(row, column));
        } else if ch == '.' {
            continue;
        } else {
            pipes.push((
                spatial::UPoint2D::from(row, column),
                RawPipe::parse(ch).unwrap().directions(),
            ));
        }
    }

    (pipes, start)
}
