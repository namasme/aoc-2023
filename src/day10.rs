use anyhow::anyhow;
use anyhow::Result;
use std::cmp;
use std::collections::HashMap;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug)]
pub struct Field {
    pub pipes: HashMap<Point2D, Pipe>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point2D {
    pub row: usize,
    pub column: usize,
}

pub type Pipe = (Direction, Direction);

#[derive(Debug, Eq, PartialEq)]
enum RawPipe {
    Horizontal,
    Vertical,
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl Field {
    pub fn trace_loop(
        &self,
        start: Point2D,
        direction: Direction,
    ) -> Vec<Point2D> {
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

    pub fn identify_start_pipe(&self, start: Point2D) -> Pipe {
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

impl Point2D {
    pub fn neighbours(&self) -> Vec<Point2D> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|direction| self.move_by(direction))
        .collect()
    }

    pub fn move_by(&self, direction: Direction) -> Option<Point2D> {
        let destination = match direction {
            Direction::Up => Point2D {
                row: self.row - 1,
                column: self.column,
            },
            Direction::Down => Point2D {
                row: self.row + 1,
                column: self.column,
            },
            Direction::Left => Point2D {
                row: self.row,
                column: self.column - 1,
            },
            Direction::Right => Point2D {
                row: self.row,
                column: self.column + 1,
            },
        };

        Some(destination).filter(Self::is_valid)
    }

    // We pad all coordinates by 1 not to deal with overflows in unsigned integers.
    fn is_valid(&self) -> bool {
        self.row > 0 && self.column > 0
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
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

    fn directions(&self) -> (Direction, Direction) {
        match self {
            RawPipe::Horizontal => (Direction::Left, Direction::Right),
            RawPipe::Vertical => (Direction::Up, Direction::Down),
            RawPipe::LeftTop => (Direction::Up, Direction::Left),
            RawPipe::RightTop => (Direction::Up, Direction::Right),
            RawPipe::LeftBottom => (Direction::Down, Direction::Left),
            RawPipe::RightBottom => (Direction::Down, Direction::Right),
        }
    }
}

pub fn parse_input(input: &str) -> (Field, Point2D) {
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

fn parse_line((row, line): (usize, &str)) -> (Vec<(Point2D, Pipe)>, Option<Point2D>) {
    let mut pipes = vec![];
    let mut start = None;
    for (column, ch) in line.chars().enumerate() {
        if ch == 'S' {
            start = Some(Point2D {
                row: row + 1,
                column: column + 1,
            });
        } else if ch == '.' {
            continue;
        } else {
            pipes.push((
                Point2D {
                    row: row + 1,
                    column: column + 1,
                },
                RawPipe::parse(ch).unwrap().directions(),
            ));
        }
    }

    (pipes, start)
}
