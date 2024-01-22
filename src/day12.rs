use std::cmp;
use std::collections::BTreeMap;
use std::iter;

// This is essentially the owned version of Arrangement
pub struct Record {
    row: String,
    lengths: Vec<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
struct Arrangement<'a> {
    row: &'a [SpringT],
    lengths: &'a [usize],
}

type SpringT = u8;
struct Spring;

impl Record {
    pub fn count_arrangements(&self) -> usize {
        let self_arrangement = Arrangement::new(self.row.as_bytes(), &self.lengths);
        let mut frequencies = BTreeMap::from([(self_arrangement, 1)]);
        let mut count = 0;
        while !frequencies.is_empty() {
            let (&arrangement, &frequency) = frequencies.iter().next().unwrap();
            frequencies.remove(&arrangement);
            arrangement
                .next()
                .into_iter()
                .for_each(|child| match child.is_satisfiable() {
                    Some(true) => count += frequency,
                    Some(false) => (),
                    None => {
                        frequencies
                            .entry(child)
                            .and_modify(|f| *f += frequency)
                            .or_insert(frequency);
                    }
                });
        }
        count
    }

    pub fn unfold(&self) -> Self {
        let row = (vec![self.row.clone(); 5]).join("?");
        let lengths = (vec![self.lengths.clone(); 5]).concat();

        Record { row, lengths }
    }

    pub fn parse(line: &str) -> Self {
        let (raw_row, raw_lengths) = line.split_once(' ').unwrap();
        let row = raw_row.to_string();
        let lengths: Vec<usize> = raw_lengths.split(',').flat_map(|s| s.parse()).collect();
        Self { row, lengths }
    }
}

impl<'a> Arrangement<'a> {
    pub fn new(row: &'a [u8], lengths: &'a [usize]) -> Arrangement<'a> {
        Self { row, lengths }.trim_operational()
    }

    fn next(&self) -> Vec<Self> {
        let mut idx = 0;
        while idx < self.row.len() && Spring::is_unknown(self.row[idx]) {
            idx += 1;
        }
        let unknown_prefix_length = idx;
        while idx < self.row.len() && Spring::is_potentially_damaged(self.row[idx]) {
            idx += 1;
        }
        let potentially_damaged_block_length = idx - unknown_prefix_length;
        // self.lengths cannot be empty because then self would have been satisfiable
        let expected_block_length = self.lengths[0];

        let unknown_prefix_dropped = iter::once(Arrangement::new(
            &self.row[unknown_prefix_length..],
            &self.lengths,
        ))
        // We only want to drop the unknown prefix if it is non-empty, so we are making progress,
        // *and* we can skip a few operational springs and try to apply the block in the next section.
        // If potentially_damaged_block_length > 0, even if we skip unknowns now we will have to try
        // to apply the block in the next iteration (because we cannot skip then).
        // If < expected_block_length, the block will not fit when we try to apply it,
        // If == expected_block_length, the block will fit perfectly, but we will already account for that
        // arrangement in block_applications.
        // // If > expected_block_length, the block will not fit in the next iteration anyway.
        .filter(|_| unknown_prefix_length > 0 && potentially_damaged_block_length == 0);

        if unknown_prefix_length + potentially_damaged_block_length < expected_block_length {
            // If the block does not fit, we can only possibly drop the unknown prefix
            return unknown_prefix_dropped.collect();
        }

        let maximum_end = unknown_prefix_length
            + cmp::min(expected_block_length, potentially_damaged_block_length);
        let block_applications = (expected_block_length..maximum_end + 1)
            .filter(|&potential_end| {
                potential_end == self.row.len()
                    || Spring::is_potentially_operational(self.row[potential_end])
            })
            .map(|potential_end| {
                if potential_end == self.row.len() {
                    Arrangement::new(&self.row[potential_end..], tail(&self.lengths))
                } else {
                    Arrangement::new(&self.row[potential_end + 1..], tail(&self.lengths))
                }
            });

        unknown_prefix_dropped.chain(block_applications).collect()
    }

    fn is_satisfiable(&self) -> Option<bool> {
        let total_expected_damaged = self.lengths.iter().sum::<usize>();
        if self.lengths.is_empty() {
            Some(!self.row.into_iter().any(|&s| Spring::is_damaged(s)))
        } else if total_expected_damaged
            > self
                .row
                .iter()
                .filter(|&&s| Spring::is_potentially_damaged(s))
                .count()
        {
            Some(false)
        } else if total_expected_damaged
            < self
                .row
                .into_iter()
                .filter(|&&s| Spring::is_damaged(s))
                .count()
        {
            Some(false)
        } else {
            None
        }
    }

    fn trim_operational(&self) -> Self {
        let mut idx = 0; // idx is 1-indexed
        while idx < self.row.len() && Spring::is_operational(self.row[idx]) {
            idx += 1;
        }

        return Self {
            row: &self.row[idx..],
            lengths: &self.lengths,
        };
    }
}

impl Ord for Arrangement<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Invert the ordering so the longest arrangements come first
        if self.row.len() == other.row.len() {
            other.partial_cmp(self).unwrap() // the derived PartialOrd is total because the underlying orders are
        } else {
            other.row.len().cmp(&self.row.len())
        }
    }
}

impl Spring {
    fn is_potentially_damaged(spring: SpringT) -> bool {
        Spring::is_damaged(spring) || Spring::is_unknown(spring)
    }

    fn is_potentially_operational(spring: SpringT) -> bool {
        Spring::is_operational(spring) || Spring::is_unknown(spring)
    }

    fn is_operational(spring: SpringT) -> bool {
        spring == b'.'
    }

    fn is_damaged(spring: SpringT) -> bool {
        spring == b'#'
    }

    fn is_unknown(spring: SpringT) -> bool {
        spring == b'?'
    }
}

pub fn parse_input(input: &str) -> Vec<Record> {
    input.lines().map(Record::parse).collect()
}

fn tail<T>(list: &[T]) -> &[T] {
    if list.is_empty() {
        list
    } else {
        &list[1..]
    }
}
