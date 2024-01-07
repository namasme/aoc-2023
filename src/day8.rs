use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use unfold::Unfold;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub type NodeID = String;

#[derive(Debug)]
pub struct Network {
    pub edges: HashMap<NodeID, (NodeID, NodeID)>,
}

#[derive(Clone, Copy)]
pub struct DirectionStream<'a> {
    base_directions: &'a [Direction],
    pub index: usize,
}

#[derive(Debug)]
pub struct Cycle {
    pub mu: u64,     // the length of the prefix
    pub lambda: u64, // the actual cycle length
}

impl Direction {
    fn parse(input: char) -> Result<Direction> {
        match input {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!(
                "could not parse the following direction: {}",
                input
            )),
        }
    }
}

impl<'a> DirectionStream<'a> {
    fn from_directions(base_directions: &'a [Direction]) -> DirectionStream<'a> {
        Self {
            base_directions,
            index: 0,
        }
    }

    fn next(&mut self) -> Direction {
        let current = self.base_directions[self.index];
        self.index = (self.index + 1) % self.base_directions.len();
        current
    }
}

impl Network {
    pub fn detect_cycle(&self, seed: &NodeID, directions: &[Direction]) -> Cycle {
        let mut tortoise_directions = DirectionStream::from_directions(directions);
        let mut hare_directions = DirectionStream::from_directions(directions);
        let mut tortoise = self.follow(seed, &mut tortoise_directions);
        let mut hare = self.follow(seed, &mut hare_directions);
        hare = self.follow(hare, &mut hare_directions);

        while (tortoise, tortoise_directions.index) != (hare, hare_directions.index) {
            tortoise = self.follow(tortoise, &mut tortoise_directions);
            hare = self.follow(hare, &mut hare_directions);
            hare = self.follow(hare, &mut hare_directions);
        }

        let mut mu = 0;
        tortoise_directions.index = 0; // reset stream
        tortoise = seed;
        while (tortoise, tortoise_directions.index) != (hare, hare_directions.index) {
            tortoise = self.follow(tortoise, &mut tortoise_directions);
            hare = self.follow(hare, &mut hare_directions);
            mu += 1;
        }

        let mut lambda = 1;
        hare_directions.index = tortoise_directions.index;
        hare = self.follow(tortoise, &mut hare_directions);
        while (tortoise, tortoise_directions.index) != (hare, hare_directions.index) {
            hare = self.follow(hare, &mut hare_directions);
            lambda += 1;
        }

        Cycle { mu, lambda }
    }

    pub fn iterate<'a>(
        &'a self,
        seed: &'a NodeID,
        directions: &'a [Direction],
    ) -> impl Iterator<Item = &'a NodeID> + '_ {
        Unfold::new(
            |(current, directions)| {
                let mut cloned = directions.clone();
                let next = self.follow(current, &mut cloned);
                (next, cloned)
            },
            (seed, DirectionStream::from_directions(directions)),
        )
        .map(|(node_id, _)| node_id)
    }

    pub fn iterate_n<'a>(
        &'a self,
        seed: &'a NodeID,
        n: usize,
        directions: &'a [Direction],
    ) -> &'a NodeID {
        self.iterate(seed, directions).skip(n).next().unwrap()
    }

    pub fn follow<'a>(
        &'a self,
        current: &'a NodeID,
        directions: &mut DirectionStream,
    ) -> &'a NodeID {
        let (left, right) = &self.edges[current];
        let direction = directions.next();

        match direction {
            Direction::Left => &left,
            Direction::Right => &right,
        }
    }
}

pub fn parse_input(input: &str) -> (Vec<Direction>, Network) {
    let (directions_block, network_block) = input.split_once("\n\n").unwrap();
    let directions = directions_block
        .chars()
        .flat_map(Direction::parse)
        .collect();
    let edges = network_block.lines().map(parse_network_line).collect();
    let network = Network { edges };
    (directions, network)
}

fn parse_network_line(input: &str) -> (NodeID, (NodeID, NodeID)) {
    let (name, edges) = input.split_once(" = ").unwrap();
    let (left, right) = edges.split_once(", ").unwrap();

    (
        name.to_string(),
        (left[1..].to_string(), right[..right.len() - 1].to_string()),
    )
}
