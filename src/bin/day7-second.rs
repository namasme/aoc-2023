use anyhow::anyhow;
use anyhow::Result;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::str;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: [Card; 5],
}

type Bid = u32;

fn main() {
    let input = fs::read_to_string("data/day7/input").unwrap();
    let mut hands = parse_input(&input);
    // Turns out [T]::sort uses T::lt, which in turn uses T::partial_cmp,
    // which is derived for Hand and simply wrong. We need to explicitly call
    // the implemented cmp from Ord instead.
    // I wonder what the idiomatic way of doing this is because it just looks odd.
    hands.sort_by(|h, h_| h.cmp(h_));
    let answer: Bid = hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| ((idx + 1) as Bid) * bid)
        .sum();
    println!("{}", answer);
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counter = HashMap::new();

        for card in self.cards {
            counter.insert(card, counter.get(&card).unwrap_or(&0) + 1);
        }

        let unique = counter.len();
        let highest_frequency = counter.values().max().unwrap();
        let jokers_count = counter.get(&Card::Joker).unwrap_or(&0);

        match (unique, highest_frequency, jokers_count) {
            (5, _, 0) => HandType::High,
            (5, _, 1) => HandType::Pair,
            (4, _, 0) => HandType::Pair,
            (4, _, _) => HandType::Three,
            (3, 2, 0) => HandType::TwoPair,
            (3, 2, 1) => HandType::Full,
            (3, 2, 2) => HandType::Four,
            (3, 3, 0) => HandType::Three,
            (3, 3, _) => HandType::Four,
            (2, 3, 0) => HandType::Full,
            (2, 3, _) => HandType::Five,
            (2, 4, 0) => HandType::Four,
            (2, 4, _) => HandType::Five,
            (1, _, _) => HandType::Five,
            (_, _, _) => panic!(
                "unexpected hand {:?} with {} unique cards and having at most {} copies of each card and {} jokers",
                self.cards, unique, highest_frequency, jokers_count
            )
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            cmp::Ordering::Equal => zip(self.cards, other.cards)
                .skip_while(|(self_card, other_card)| self_card == other_card)
                .next()
                .map(|(self_card, other_card)| self_card.cmp(&other_card))
                .unwrap_or(cmp::Ordering::Equal),
            other => other,
        }
    }
}

impl str::FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Hand> {
        Ok(to_hand(
            input.chars().take(5).flat_map(Card::parse).collect(),
        ))
    }
}

// A bit hacky but maintains the length invariant at the type level.
fn to_hand(cards: Vec<Card>) -> Hand {
    Hand {
        cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
    }
}

impl Card {
    fn parse(input: char) -> Result<Card> {
        match input {
            'J' => Ok(Card::Joker),
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(anyhow!("could not parse the following card: {}", input)),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Hand, Bid)> {
    input.lines().flat_map(parse_line).collect()
}

fn parse_line(input: &str) -> Result<(Hand, Bid)> {
    input
        .split_once(' ')
        .map(|(raw_hand, raw_bid)| Ok((raw_hand.parse()?, raw_bid.parse()?)))
        .unwrap_or(Err(anyhow!(
            "could not parse the following line: {}",
            input
        )))
}
