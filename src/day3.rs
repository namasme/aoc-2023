use std::cmp;
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Schematic {
    pub numbers: Vec<Number>,
    pub symbols: HashMap<Position, Symbol>,
}

#[derive(Clone, Copy, Debug)]
pub struct Number {
    pub value: u32,
    pub position: Position, // leftmost
    pub length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    row: usize,
    column: usize,
}

type Symbol = char;

pub fn perimeter(number: Number) -> Vec<Position> {
    let r = number.position.row;
    let c = number.position.column;
    let top_left = Position {
        row: r.saturating_sub(1),
        column: c.saturating_sub(1),
    };
    let bottom_left = Position {
        row: r + 1,
        column: c.saturating_sub(1),
    };
    let top_right = Position {
        row: r.saturating_sub(1),
        column: c + number.length,
    };
    let bottom_right = Position {
        row: r + 1,
        column: c + number.length,
    };
    let mut result = vec![];
    //result.append(&mut segment(top_left, bottom_left));
    result.push(Position {
        row: r,
        column: c.saturating_sub(1),
    });
    result.append(&mut segment(top_left, top_right));
    result.append(&mut segment(bottom_left, bottom_right));
    result.push(Position {
        row: r,
        column: c + number.length,
    });
    //result.append(&mut segment(top_right, bottom_right));
    result
}

fn segment(a: Position, b: Position) -> Vec<Position> {
    if a.row == b.row {
        let min = cmp::min(a.column, b.column);
        let max = cmp::max(a.column, b.column);
        (min..=max)
            .map(|column| Position { row: a.row, column })
            .collect()
    } else if a.column == b.column {
        let min = cmp::min(a.row, b.row);
        let max = cmp::max(a.row, b.row);
        (min..=max)
            .map(|row| Position {
                row,
                column: a.column,
            })
            .collect()
    } else {
        panic!("unable to generate segment")
    }
}

pub fn parse_input(raw_input: &str) -> Schematic {
    let mut numbers = vec![];
    let mut symbols = HashMap::new();

    for (row, line) in raw_input.lines().enumerate() {
        let mut chars = line.chars().enumerate().peekable();
        loop {
            match chars.peek() {
                None => break,
                Some((_, '.')) => {
                    chars.next();
                }
                Some((_, ch)) if ch.is_digit(10) => numbers.push(consume_number(&mut chars, row)),
                Some((column, ch)) => {
                    symbols.insert(
                        Position {
                            row,
                            column: *column,
                        },
                        *ch,
                    );
                    chars.next();
                }
            }
        }
    }

    Schematic { numbers, symbols }
}

fn consume_number(chars: &mut Peekable<impl Iterator<Item = (usize, char)>>, row: usize) -> Number {
    let mut indexed_digits: Vec<(usize, char)> = vec![];
    while chars.peek().is_some_and(|(_, ch)| ch.is_digit(10)) {
        indexed_digits.push(chars.next().unwrap());
    }
    let length = indexed_digits.len();
    let column = indexed_digits[0].0;
    let value = indexed_digits
        .iter()
        .map(|digit| digit.1)
        .collect::<String>()
        .parse()
        .unwrap();
    Number {
        value,
        position: Position { row, column },
        length,
    }
}
