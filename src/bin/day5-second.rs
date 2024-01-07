use aoc_2023::day5;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day5/input").unwrap();
    let (ranges, maps) = parse_input(&input);

    let final_map = maps
        .iter()
        .map(saturate)
        .reduce(SaturatedMap::and_then)
        .unwrap();
    let minimum = ranges
        .iter()
        .flat_map(|range| explode_range(range, &final_map))
        .map(|seed| final_map.apply(seed))
        .min()
        .unwrap();
    println!("answer: {}", minimum);
}

// Equivalent to Map, but explicitly claims the saturation property.
// That means there are no gaps in the pieces (except at the end,
// because infinity does not fit in a u64) and they are sorted.
struct SaturatedMap {
    pieces: Vec<day5::Piece>,
}

fn explode_range(range: &Range, map: &SaturatedMap) -> Vec<day5::Identifier> {
    let mut current = range.from;
    let mut breakpoints = vec![];

    while current < range.from + range.size {
        breakpoints.push(current);
        match map.locate_piece(current) {
            Some((_, piece)) => current = piece.from + piece.size,
            None => break,
        }
    }

    breakpoints
}

fn saturate(map: &day5::Map) -> SaturatedMap {
    let mut sorted = map.pieces.clone();
    sorted.sort();
    if sorted[0].from > 0 {
        sorted.insert(
            0,
            day5::Piece {
                from: 0,
                to: 0,
                size: sorted[0].from,
            },
        )
    }

    let mut new_pieces = Vec::with_capacity(sorted.len());
    for idx in 0..sorted.len() {
        new_pieces.push(sorted[idx].clone());
        let is_gap =
            idx + 1 < sorted.len() && sorted[idx].from + sorted[idx].size < sorted[idx + 1].from;
        if is_gap {
            let gap_start = sorted[idx].from + sorted[idx].size;
            new_pieces.push(day5::Piece {
                from: gap_start,
                to: gap_start,
                size: sorted[idx + 1].from - gap_start,
            });
        }
    }

    SaturatedMap { pieces: new_pieces }
}

impl SaturatedMap {
    fn apply(&self, seed: day5::Identifier) -> day5::Identifier {
        self.locate_piece(seed)
            .map(|(_, piece)| piece.to + (seed - piece.from))
            .unwrap_or(seed)
    }

    // TODO: could use binary search
    fn locate_piece(&self, target: day5::Identifier) -> Option<(usize, &day5::Piece)> {
        self.pieces
            .iter()
            .enumerate()
            .find(|(_, piece)| piece.from <= target && target < piece.from + piece.size)
    }

    fn and_then(self, other: SaturatedMap) -> SaturatedMap {
        let mut pieces = vec![];

        let mut self_idx = 0;
        let mut current = 0;
        let end = self.pieces[self.pieces.len() - 1].from + self.pieces[self.pieces.len() - 1].size;

        while current < end {
            while self_idx < self.pieces.len()
                && current >= self.pieces[self_idx].from + self.pieces[self_idx].size
            {
                self_idx += 1;
            }
            let destination = self.pieces[self_idx].to + (current - self.pieces[self_idx].from);
            let other_piece = other.locate_piece(destination);
            let new_piece;
            (current, new_piece) = match other_piece {
                Some((_, other_piece)) => {
                    compose_pieces(current, &self.pieces[self_idx], &other_piece)
                }
                None => (
                    self.pieces[self_idx].from + self.pieces[self_idx].size,
                    day5::Piece {
                        from: current,
                        to: self.pieces[self_idx].to + (current - self.pieces[self_idx].from),
                        size: self.pieces[self_idx].size - (current - self.pieces[self_idx].from),
                    },
                ),
            };
            pieces.push(new_piece);
        }

        let other_remaining = other.locate_piece(end);
        if let Some((other_idx, other_piece)) = other_remaining {
            pieces.push(day5::Piece {
                from: end,
                to: other_piece.to + end - other_piece.from,
                size: other_piece.size - (end - other_piece.from),
            });
            pieces.extend_from_slice(&other.pieces[other_idx + 1..]);
        }

        // TODO: contiguous pieces might be redundant
        // e.g. 1 0 5, 6 5 10 -> 1 0 15
        SaturatedMap { pieces }
    }
}

fn compose_pieces(
    seed: day5::Identifier,
    first: &day5::Piece,
    second: &day5::Piece,
) -> (day5::Identifier, day5::Piece) {
    let destination = first.to + (seed - first.from);
    if first.to + first.size < second.from + second.size {
        (
            first.from + first.size,
            day5::Piece {
                from: seed,
                to: second.to + (destination - second.from),
                size: first.size - (seed - first.from),
            },
        )
    } else {
        (
            seed + second.size - (destination - second.from),
            day5::Piece {
                from: seed,
                to: second.to + (destination - second.from),
                size: second.size - (destination - second.from),
            },
        )
    }
}

struct Range {
    from: day5::Identifier,
    size: day5::Identifier,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<day5::Map>) {
    let (ranges_block, rest) = input.split_once("\n\n").unwrap();
    let raw_ranges: Vec<day5::Identifier> = ranges_block
        .split(' ')
        .skip(1)
        .flat_map(str::parse)
        .collect();

    let ranges = (0..raw_ranges.len())
        .step_by(2)
        .map(|idx| Range {
            from: raw_ranges[idx],
            size: raw_ranges[idx + 1],
        })
        .collect();

    let maps = rest.split("\n\n").flat_map(str::parse).collect();

    (ranges, maps)
}
