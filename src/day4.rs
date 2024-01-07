use std::collections;
use std::str;

type Number = u8;
pub struct Scratchcard {
    winning: collections::HashSet<Number>,
    present: collections::HashSet<Number>,
}

impl Scratchcard {
    pub fn count_matches(&self) -> usize {
        self
            .winning
            .intersection(&self.present)
            .collect::<Vec<_>>()
            .len()
    }

    pub fn score(&self) -> u64 {
        let matches_count = self.count_matches();

        if matches_count == 0 {
            0
        } else {
            1 << (matches_count - 1)
        }
    }
}

pub struct ParseScratchcardErr;
impl str::FromStr for Scratchcard {
    type Err = ParseScratchcardErr;
    fn from_str(input: &str) -> Result<Scratchcard, Self::Err> {
        let (raw_winning, raw_present) = input.split_once(':').unwrap().1.split_once('|').unwrap();
        Ok(Scratchcard {
            winning: raw_winning
                .split_ascii_whitespace()
                .flat_map(str::parse)
                .collect(),
            present: raw_present
                .split_ascii_whitespace()
                .flat_map(str::parse)
                .collect(),
        })
    }
}
