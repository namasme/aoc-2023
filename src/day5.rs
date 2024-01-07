use std::str;

#[derive(Debug)]
pub struct Map {
    pub pieces: Vec<Piece>,
}

pub type Identifier = u64;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Piece {
    pub from: Identifier,
    pub to: Identifier,
    pub size: Identifier,
}

impl Map {
    pub fn apply(&self, seed: Identifier) -> Identifier {
        self.pieces
            .iter()
            .find(|piece| piece.from <= seed && seed < piece.from + piece.size)
            .map(|piece| piece.to + (seed - piece.from))
            .unwrap_or(seed)
    }
}

pub struct ParseMapErr;
impl str::FromStr for Map {
    type Err = ParseMapErr;
    fn from_str(input: &str) -> Result<Map, Self::Err> {
        Ok(Map {
            pieces: input.lines().skip(1).flat_map(str::parse).collect(),
        })
    }
}

pub struct ParsePieceErr;
impl str::FromStr for Piece {
    type Err = ParsePieceErr;
    fn from_str(input: &str) -> Result<Piece, Self::Err> {
        let nums: Vec<Identifier> = input.split(' ').flat_map(str::parse).collect();
        match nums[..] {
            [to, from, size] => Ok(Piece { from, to, size }),
            _ => Err(ParsePieceErr),
        }
    }
}
