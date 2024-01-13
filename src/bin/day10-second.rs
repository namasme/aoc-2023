use aoc_2023::day10;
use aoc_2023::spatial;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Range {
    start: usize,
    size: usize,
}

#[derive(Debug)]
struct ClosedLoop {
    path: Vec<spatial::UPoint2D>,
    pipes: HashMap<spatial::UPoint2D, day10::Pipe>,
}

fn main() {
    let input = fs::read_to_string("data/day10/input").unwrap();
    let (field, start) = day10::parse_input(&input);
    let closed_loop = ClosedLoop::trace_for(&field, start);
    let answer = closed_loop.area();
    println!("{answer}");
}

impl ClosedLoop {
    fn trace_for(field: &day10::Field, start: spatial::UPoint2D) -> Self {
        let path = field.trace_loop(start, field.identify_start_pipe(start).0);
        let pipes = path
            .iter()
            .map(|&point| {
                if point == start {
                    (point, field.identify_start_pipe(start))
                } else {
                    (point, field.pipes[&point])
                }
            })
            .collect();
        Self { path, pipes }
    }

    fn area(&self) -> u64 {
        let by_rows = self.by_rows();

        by_rows
            .iter()
            .map(|(&row, ranges)| self.row_area(row, &ranges))
            .sum::<usize>() as u64
    }

    fn by_rows(&self) -> HashMap<usize, Vec<Range>> {
        let mut path = self.path.clone(); // TODO
        path.sort_unstable();
        let mut by_rows = HashMap::new();
        let mut idx = 0;

        while idx < path.len() {
            let spatial::UPoint2D { row, column } = path[idx];
            let start_idx = idx;
            while idx < path.len()
                && path[idx].row == row
                && continues_rightward(self.pipes[&path[idx]])
            {
                idx += 1;
            }
            idx += 1;
            by_rows.entry(row).or_insert(vec![]).push(Range {
                start: column,
                size: idx - start_idx,
            });
        }

        by_rows
            .iter_mut()
            .for_each(|(_, ranges)| ranges.sort_unstable_by_key(|range| range.start));

        by_rows
    }

    fn row_area(&self, row: usize, ranges: &[Range]) -> usize {
        let pipe_at = |column| self.pipes[&spatial::UPoint2D { row, column }];
        let mut inside = false;
        let mut accumulated_miles: usize = 0;

        for idx in 0..ranges.len() {
            if inside {
                accumulated_miles += ranges[idx].start - ranges[idx - 1].end();
            }
            if ranges[idx].size > 1 {
                if continues_rightward(pipe_at(ranges[idx].start)) {
                    inside = self.update_inside(
                        inside,
                        row,
                        &ranges[idx],
                    );
                }
            } else {
                inside = !inside;
            }
        }
        accumulated_miles
    }

    fn update_inside(&self, current_inside: bool, row: usize, range: &Range) -> bool {
        // When we run into a horizontal wall, we need to decide whether
        // we are inside or outside the loop after it. This depends on the
        // current value of inside and the direction the wall came from
        // and continues towards.
        //
        // For instance, if we find this wall (looking only at the line in the middle)
        //
        // IIIIIII | OO
        // II F----J OO
        // II | OOOOOOO
        //
        // If the left end is inside the loop, then the right end is outside
        // because the wall came from below but continues upwards, so we invert
        // its value.
        //
        // If on the other hand, the left and right ends of the wall come
        // from the same direction, then inside is unchanged, e.g.
        //
        // IIIIIIIIIIII
        // II F----7 II
        // II | OO | II
        //
        // or
        //
        // OO | II | OO
        // OO F----7 OO
        // OOOOOOOOOOOO
        let left_end = self.pipes[&spatial::UPoint2D { row, column: range.start }];
        let right_end = self.pipes[&spatial::UPoint2D { row, column: range.end() - 1 }];
        current_inside == (
            came_from(left_end) == came_from(right_end)
        )
    }
}

impl Range {
    fn end(&self) -> usize {
        self.start + self.size
    }
}

fn came_from(pipe: day10::Pipe) -> spatial::Direction {
    pipe.0
}

fn continues_rightward(pipe: day10::Pipe) -> bool {
    pipe.1 == spatial::Direction::Right
}
