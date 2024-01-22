type AxisSummary = u32;
#[derive(Debug)]
pub struct Pattern {
    rows: Vec<AxisSummary>,
    columns: Vec<AxisSummary>,
}

#[derive(Debug)]
pub enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Pattern {
    pub fn find_smudged_reflection(&self) -> Reflection {
        let horizontal_candidates = (1..self.rows.len())
            .filter(|&split_at| Self::is_reflected_with_smudge_at(&self.rows, split_at))
            .map(|split_at| Reflection::Horizontal(split_at));
        let vertical_candidates = (1..self.columns.len())
            .filter(|&split_at| Self::is_reflected_with_smudge_at(&self.columns, split_at))
            .map(|split_at| Reflection::Vertical(split_at));

        horizontal_candidates
            .chain(vertical_candidates)
            .next()
            .unwrap()
    }

    pub fn find_reflection(&self) -> Reflection {
        let horizontal_candidates = (1..self.rows.len())
            .filter(|&split_at| Self::is_reflected_at(&self.rows, split_at))
            .map(|split_at| Reflection::Horizontal(split_at));
        let vertical_candidates = (1..self.columns.len())
            .filter(|&split_at| Self::is_reflected_at(&self.columns, split_at))
            .map(|split_at| Reflection::Vertical(split_at));

        horizontal_candidates
            .chain(vertical_candidates)
            .next()
            .unwrap()
    }

    fn is_reflected_with_smudge_at(axis: &[AxisSummary], index: usize) -> bool {
        let (left, right) = Self::compute_ends(index, axis.len());

        Self::required_smudges(&axis[left..right]) == 1
    }

    // Calculates how many smudges would be required to make the axis a palindrome.
    // The xor between two axis summaries indicates how many bits are different.
    // count_ones then returns how many bits are set, or how many smudges are needed.
    fn required_smudges(axis: &[AxisSummary]) -> usize {
        (0..axis.len() / 2)
            .map(|idx| (axis[idx] ^ axis[axis.len() - idx - 1]).count_ones() as usize)
            .sum()
    }

    fn is_reflected_at(axis: &[AxisSummary], index: usize) -> bool {
        let (left, right) = Self::compute_ends(index, axis.len());
        Self::is_palindrome(&axis[left..right])
    }

    fn compute_ends(candidate_index: usize, axis_length: usize) -> (usize, usize) {
        if candidate_index + candidate_index < axis_length {
            (0, candidate_index + candidate_index)
        } else {
            // If candidate_index + candidate_index >= axis_length, we are past the halfway point.
            // The right end should be axis_length, and the left end should be
            // axis_length - (axis_length - candidate_index) - (axis_length - candidate_index),
            // which simplifies to candidate_index + candidate_index - axis_length.
            (candidate_index + candidate_index - axis_length, axis_length)
        }
    }

    fn is_palindrome(axis: &[AxisSummary]) -> bool {
        axis.iter()
            .zip(axis.iter().rev())
            .all(|(left, right)| left == right)
    }

    fn parse(input: &str) -> Self {
        let rows = Self::parse_rows(input);
        let columns = Self::parse_columns(input);
        Self { rows, columns }
    }

    fn parse_rows(input: &str) -> Vec<AxisSummary> {
        input
            .lines()
            .map(str::chars)
            .map(Self::parse_axis_summary)
            .collect()
    }

    fn parse_columns(input: &str) -> Vec<AxisSummary> {
        // Build a Vec of iterators over the characters of each line
        let mut line_iterators: Vec<_> = input.lines().map(str::chars).collect();
        let columns_count = input.lines().next().unwrap().len();
        (0..columns_count)
            .map(|_| {
                // Build an iterator such that each element is the next character of the corresponding
                // line iterator, which is consumed in the process, and apply parse_axis_summary to it.
                // This way we first parse the first character of each line, which is the first column,
                // then the second one, and so on. It's a convoluted way of transposing the input.
                Self::parse_axis_summary(line_iterators.iter_mut().map(|line| line.next().unwrap()))
            })
            .collect()
    }

    fn parse_axis_summary(axis: impl Iterator<Item = char>) -> AxisSummary {
        axis.fold(0, |acc, c| (acc << 1) | Self::parse_bit(c))
    }

    fn parse_bit(input: char) -> AxisSummary {
        match input {
            '#' => 1,
            '.' => 0,
            _ => panic!("Invalid character"),
        }
    }
}

impl Reflection {
    pub fn summarize(&self) -> usize {
        match self {
            Reflection::Horizontal(rows_count) => 100 * rows_count,
            Reflection::Vertical(columns_count) => *columns_count,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::parse).collect()
}
