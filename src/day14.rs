use crate::common::CycleDetection;
use crate::common::FiniteCycleIter;
use crate::spatial::Direction;
use std::iter::once;
use std::iter::zip;
use std::ops::Range;

#[derive(Clone, PartialEq, Eq)]
pub struct Platform {
    cubed: Cubed,
    rounded: Rounded,
}

pub struct PlatformIter {
    platform: Platform,
    direction: FiniteCycleIter<Direction>,
}

#[derive(Clone, PartialEq, Eq)]
struct Cubed {
    rows: Vec<CubedAxis>,
    columns: Vec<CubedAxis>,
}

#[derive(Clone, PartialEq, Eq)]
struct Rounded {
    rows: Vec<RoundedAxis>,
    columns: Vec<RoundedAxis>,
}

type RoundedAxis = Vec<Position>;
type CubedAxis = Vec<Position>;

type Position = usize;
type Span = fn(Position, Position, usize) -> Range<Position>;

impl Platform {
    pub fn run_for(self, tilts_count: usize) -> Platform {
        let cycle = self.detect_cycle();

        if tilts_count < cycle.mu {
            self.into_iter().skip(tilts_count).next().unwrap()
        } else {
            self.into_iter()
                .skip(cycle.mu)
                .skip((tilts_count - cycle.mu) % cycle.lambda)
                .next()
                .unwrap()
        }
    }

    pub fn load(&self) -> usize {
        self.rounded
            .rows
            .iter()
            .enumerate()
            .map(|(row_idx, rounded)| (self.rounded.rows.len() - row_idx) * rounded.len())
            .sum()
    }

    pub fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.apply_tilt(Self::tilt_by_columns, Self::span_left),
            Direction::Left => self.apply_tilt(Self::tilt_by_rows, Self::span_left),
            Direction::Down => self.apply_tilt(Self::tilt_by_columns, Self::span_right),
            Direction::Right => self.apply_tilt(Self::tilt_by_rows, Self::span_right),
        }
    }

    fn apply_tilt(&mut self, tilt_by: fn(&Self, Span) -> Rounded, span: Span) {
        self.rounded = tilt_by(&self, span);
    }

    fn tilt_by_rows(&self, span: Span) -> Rounded {
        let rows = self.tilt_axes(
            &self.cubed.rows,
            &self.rounded.rows,
            self.cubed.rows.len(),
            span,
        );
        let columns = Rounded::transpose(&rows);
        Rounded { rows, columns }
    }

    fn tilt_by_columns(&self, span: Span) -> Rounded {
        let columns = self.tilt_axes(
            &self.cubed.columns,
            &self.rounded.columns,
            self.cubed.columns.len(),
            span,
        );
        let rows = Rounded::transpose(&columns);
        Rounded { rows, columns }
    }

    fn tilt_axes(
        &self,
        cubed: &[CubedAxis],
        rounded: &[RoundedAxis],
        axis_length: usize,
        span: Span,
    ) -> Vec<RoundedAxis> {
        cubed
            .iter()
            .zip(rounded.iter())
            .map(|(cubed, rounded)| self.tilt_axis(&cubed, &rounded, axis_length, span))
            .collect()
    }

    fn tilt_axis(
        &self,
        cubed_axis: &CubedAxis,
        rounded_axis: &RoundedAxis,
        axis_length: usize,
        span: Span,
    ) -> RoundedAxis {
        let lowers = once(0).chain(cubed_axis.iter().map(|position| position + 1));
        let uppers = cubed_axis.iter().cloned().chain(once(axis_length));

        zip(
            zip(lowers, uppers),
            self.count_by_section(cubed_axis, rounded_axis),
        )
        // count is the number of rounded rocks in the current section,
        // between lower and upper
        .flat_map(|((lower, upper), count)| span(lower, upper, count))
        .collect()
    }

    fn span_left(lower: Position, _: Position, count: usize) -> Range<Position> {
        lower..lower + count
    }

    fn span_right(_: Position, upper: Position, count: usize) -> Range<Position> {
        upper - count..upper
    }

    fn count_by_section(&self, cubed_axis: &CubedAxis, rounded_axis: &RoundedAxis) -> Vec<usize> {
        //let mut counts = vec![0; cubed_axis.len() + 1]; //
        let mut counts = Vec::with_capacity(cubed_axis.len() + 1);
        let mut rounded_idx = 0;

        for cubed_position in cubed_axis {
            let current_count = rounded_axis[rounded_idx..]
                .iter()
                .take_while(|&rounded_position| rounded_position < cubed_position)
                .count();
            rounded_idx += current_count;
            counts.push(current_count);
        }

        // ran out of cubed for the rest of the axis
        counts.push(rounded_axis.len() - rounded_idx); // just count the remaining rounded

        counts
    }
}

impl IntoIterator for Platform {
    type Item = Platform;
    type IntoIter = PlatformIter;
    fn into_iter(self) -> Self::IntoIter {
        PlatformIter {
            platform: self,
            direction: FiniteCycleIter {
                values: vec![
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                ],
                index: 0,
            },
        }
    }
}

impl Iterator for PlatformIter {
    type Item = Platform;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.platform.clone();
        let direction = self.direction.next();
        self.platform.tilt(direction?);
        Some(current)
    }
}

impl Cubed {
    fn transpose(axes: &Vec<CubedAxis>) -> Vec<CubedAxis> {
        let mut transposed = vec![vec![]; axes.len()]; // Assume same length
        for (row, axis) in axes.iter().enumerate() {
            for &position in axis.iter() {
                transposed[position].push(row);
            }
        }

        transposed
    }
}

impl Rounded {
    fn transpose(axes: &Vec<RoundedAxis>) -> Vec<RoundedAxis> {
        let mut transposed = vec![vec![]; axes.len()]; // Assume same length
        for (row, axis) in axes.iter().enumerate() {
            for &position in axis.iter() {
                transposed[position].push(row);
            }
        }

        transposed
    }
}

pub fn parse_input(input: &str) -> Platform {
    let (rows_cubed, rows_rounded) = input.lines().map(parse_row).unzip();
    let columns_cubed = Cubed::transpose(&rows_cubed);
    let columns_rounded = Rounded::transpose(&rows_rounded);

    Platform {
        cubed: Cubed {
            rows: rows_cubed,
            columns: columns_cubed,
        },
        rounded: Rounded {
            rows: rows_rounded,
            columns: columns_rounded,
        },
    }
}

fn parse_row(line: &str) -> (CubedAxis, RoundedAxis) {
    line.chars().enumerate().fold(
        (vec![], vec![]),
        |(mut cubed, mut rounded), (idx, c)| match c {
            'O' => {
                rounded.push(idx);
                (cubed, rounded)
            }
            '#' => {
                cubed.push(idx);
                (cubed, rounded)
            }
            '.' => (cubed, rounded),
            _ => panic!("Invalid character"),
        },
    )
}
