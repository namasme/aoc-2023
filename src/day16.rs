use crate::spatial::Direction;
use crate::spatial::UPoint2D;
use either::Either;
use either::Left;
use either::Right;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mirror {
    Horizontal,   // -
    Vertical,     // |
    Diagonal,     // \
    Antidiagonal, // /
}

pub struct Contraption {
    mirrors: HashMap<UPoint2D, Mirror>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Beam {
    pub position: UPoint2D,
    pub direction: Direction,
}

impl Contraption {
    pub fn maximize_energized_tiles(&self) -> usize {
        let top = (0..self.width).map(|column| Beam {
            position: UPoint2D::from(0, column),
            direction: Direction::Down,
        });
        let bottom = (0..self.width).map(|column| Beam {
            position: UPoint2D::from(self.height - 1, column),
            direction: Direction::Up,
        });
        let left = (0..self.height).map(|row| Beam {
            position: UPoint2D::from(row, 0),
            direction: Direction::Right,
        });
        let right = (0..self.height).map(|row| Beam {
            position: UPoint2D::from(row, self.width - 1),
            direction: Direction::Left,
        });

        top.chain(bottom)
            .chain(left)
            .chain(right)
            .map(|seed| self.count_energized_tiles(seed))
            .max()
            .unwrap()
    }

    pub fn count_energized_tiles(&self, seed: Beam) -> usize {
        self.beam_path(seed).len()
    }

    pub fn parse(input: &str) -> Self {
        let mirrors = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices()
                    .map(move |(column, tile)| ((row, column), tile))
            })
            .filter(|(_, tile)| tile != &'.')
            .map(|((row, column), tile)| (UPoint2D::from(row, column), Mirror::parse(tile)))
            .collect();

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        Self {
            mirrors,
            width,
            height,
        }
    }

    fn beam_path(&self, seed: Beam) -> HashSet<UPoint2D> {
        let mut visited = HashSet::new();
        let mut queue = vec![seed];

        while let Some(beam) = queue.pop() {
            if visited.contains(&beam) {
                continue;
            }

            visited.insert(beam);

            let next_direction = self
                .mirrors
                .get(&beam.position)
                .map(|mirror| mirror.reflect(beam.direction))
                .unwrap_or(Left(beam.direction));

            let directions = match next_direction {
                Left(d) => vec![d],
                Right((d1, d2)) => vec![d1, d2],
            };

            queue.extend(
                directions
                    .into_iter()
                    .flat_map(|d| beam.advance(d).into_iter())
                    .filter(|beam| self.is_valid(beam.position)),
            );
        }

        visited.iter().map(|beam| beam.position).collect()
    }

    fn is_valid(&self, position: UPoint2D) -> bool {
        position.within_bounds(self.width, self.height)
    }
}

impl Beam {
    fn advance(&self, direction: Direction) -> Option<Beam> {
        self.position.move_by(direction).map(|position| Beam {
            position,
            direction,
        })
    }
}

impl Mirror {
    fn parse(tile: char) -> Mirror {
        match tile {
            '-' => Mirror::Horizontal,
            '|' => Mirror::Vertical,
            '\\' => Mirror::Diagonal,
            '/' => Mirror::Antidiagonal,
            _ => panic!("Unexpected tile: {}", tile),
        }
    }

    fn reflect(self, direction: Direction) -> Either<Direction, (Direction, Direction)> {
        match self {
            Mirror::Horizontal => match direction {
                Direction::Up => Right((Direction::Left, Direction::Right)),
                Direction::Down => Right((Direction::Left, Direction::Right)),
                _ => Left(direction),
            },
            Mirror::Vertical => match direction {
                Direction::Left => Right((Direction::Up, Direction::Down)),
                Direction::Right => Right((Direction::Up, Direction::Down)),
                _ => Left(direction),
            },
            Mirror::Diagonal => match direction {
                Direction::Up => Left(Direction::Left),
                Direction::Down => Left(Direction::Right),
                Direction::Left => Left(Direction::Up),
                Direction::Right => Left(Direction::Down),
            },
            Mirror::Antidiagonal => match direction {
                Direction::Up => Left(Direction::Right),
                Direction::Down => Left(Direction::Left),
                Direction::Left => Left(Direction::Down),
                Direction::Right => Left(Direction::Up),
            },
        }
    }
}
