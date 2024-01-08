pub type Value = i64;

pub struct History {
    values: Vec<Value>,
}

impl History {
    pub fn predict_previous(&mut self) -> Value {
        let mut is_constant = false;
        let mut left = 0;

        while !is_constant {
            left += 1;
            for idx in (left..self.values.len()).rev() {
                self.values[idx] = self.values[idx] - self.values[idx - 1];
            }

            let last = self.values[self.values.len() - 1];
            is_constant = self.values.iter().skip(left).all(|&x| x == last);
        }

        self.values
            .iter()
            .enumerate()
            .take(left + 1)
            .map(|(idx, &value)| if idx % 2 == 0 { value } else { -value })
            .sum()
    }

    pub fn predict_next(&mut self) -> Value {
        let mut is_constant = false;
        let mut right = self.values.len();

        while !is_constant {
            right -= 1;
            for idx in 0..right {
                self.values[idx] = self.values[idx + 1] - self.values[idx];
            }

            let first = self.values[0];
            is_constant = self.values.iter().take(right).all(|&x| x == first) || right <= 0;
        }

        self.values.iter().skip(right - 1).sum()
    }
}

pub fn parse_input(input: &str) -> Vec<History> {
    input.lines().map(parse_history).collect()
}

fn parse_history(raw_history: &str) -> History {
    History {
        values: raw_history.split(' ').flat_map(str::parse).collect(),
    }
}
