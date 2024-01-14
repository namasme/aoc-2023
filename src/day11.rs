use crate::spatial::UPoint2D;
use std::cmp;

type Distance = u64;
type Position = usize;
type Galaxy = UPoint2D;

#[derive(Debug)]
pub struct Image {
    galaxies: Vec<Galaxy>,
}

type Axis = Vec<Position>;

impl Image {
    pub fn total_distance(&self, dark_energy: Distance) -> Distance {
        let rows = self.rows();
        let columns = self.columns();
        let mut accumulated = 0;

        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                let from = &self.galaxies[i];
                let to = &self.galaxies[j];
                accumulated += Image::distance_between(from, to, &rows, &columns, dark_energy);
            }
        }

        accumulated
    }

    fn distance_between(
        from: &Galaxy,
        to: &Galaxy,
        rows: &Axis,
        columns: &Axis,
        dark_energy: Distance,
    ) -> Distance {
        (from.row as isize - to.row as isize).abs() as u64
            + (from.column as isize - to.column as isize).abs() as u64
            + (dark_energy - 1)
                * Image::count_empty(cmp::min(from.row, to.row), cmp::max(from.row, to.row), rows)
            + (dark_energy - 1)
                * Image::count_empty(
                    cmp::min(from.column, to.column),
                    cmp::max(from.column, to.column),
                    columns,
                )
    }

    fn count_empty(from: Position, to: Position, axis: &Axis) -> u64 {
        let mut idx = 0;

        while idx < axis.len() && axis[idx] <= from {
            idx += 1;
        }

        let mut count = 0;
        while idx < axis.len() && axis[idx] < to {
            count += 1;
            idx += 1;
        }

        // If to == from or to == from + 1, it should be 0
        let between = (to - from).saturating_sub(1);
        (between - count) as u64
    }

    fn rows(&self) -> Axis {
        let mut idx = 0;
        let mut rows = vec![];

        while idx < self.galaxies.len() {
            let row = self.galaxies[idx].row;
            rows.push(row);
            idx += 1;

            while idx < self.galaxies.len() && self.galaxies[idx].row == row {
                idx += 1;
            }
        }

        rows
    }

    fn columns(&self) -> Axis {
        let mut columns = self
            .galaxies
            .iter()
            .map(|galaxy| galaxy.column)
            .collect::<Vec<_>>();
        columns.sort_unstable();
        columns.dedup();
        columns
    }
}

pub fn parse_input(input: &str) -> Image {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(column, char)| {
                if char == '#' {
                    Some(UPoint2D::from(row, column))
                } else {
                    None
                }
            })
        })
        .collect();
    Image { galaxies }
}
