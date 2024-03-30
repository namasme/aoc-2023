use std::ops::Neg;
use std::ops::Sub;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UPoint2D {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Orientation {
    Clockwise,
    Counterclockwise,
}

impl UPoint2D {
    pub fn from(row: usize, column: usize) -> Self {
        // We pad all coordinates by 1 not to deal with overflows in unsigned integers.
        Self {
            row: row + 1,
            column: column + 1,
        }
    }
    pub fn neighbours(&self) -> Vec<UPoint2D> {
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

    pub fn move_by(&self, direction: Direction) -> Option<UPoint2D> {
        let destination = match direction {
            Direction::Up => UPoint2D {
                row: self.row - 1,
                column: self.column,
            },
            Direction::Down => UPoint2D {
                row: self.row + 1,
                column: self.column,
            },
            Direction::Left => UPoint2D {
                row: self.row,
                column: self.column - 1,
            },
            Direction::Right => UPoint2D {
                row: self.row,
                column: self.column + 1,
            },
        };

        Some(destination).filter(Self::is_valid)
    }

    pub fn within_bounds(&self, width: usize, height: usize) -> bool {
        self.is_valid() && self.column <= width && self.row <= height
    }

    pub fn is_valid(&self) -> bool {
        self.row > 0 && self.column > 0
    }
}

impl Direction {
    pub fn rotate(self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Clockwise => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Orientation::Counterclockwise => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
        }
    }
}

impl Sub for UPoint2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
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
