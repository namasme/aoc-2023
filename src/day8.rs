use crate::common::Cycle;
use crate::common::CycleDetection; // importing to have access to the Iterator instance
use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub type NodeID = String;

#[derive(Debug, Eq, PartialEq)]
pub struct Network {
    pub edges: HashMap<NodeID, (NodeID, NodeID)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectionStream<'a> {
    base_directions: &'a [Direction],
    pub index: usize,
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
    pub fn from_directions(base_directions: &'a [Direction]) -> DirectionStream<'a> {
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
        let step = Step {
            node_id: seed,
            network: self,
            direction_stream: DirectionStream::from_directions(directions),
        };

        step.detect_cycle()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step<'a> {
    pub node_id: &'a NodeID,
    pub network: &'a Network,
    pub direction_stream: DirectionStream<'a>,
}

pub struct StepIter<'a> {
    step: Step<'a>,
}

impl<'a> IntoIterator for Step<'a> {
    type Item = Step<'a>;
    type IntoIter = StepIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        StepIter { step: self.clone() }
    }
}

impl<'a> Iterator for StepIter<'a> {
    type Item = Step<'a>;
    fn next(&mut self) -> Option<Step<'a>> {
        let (left, right) = &self.step.network.edges[self.step.node_id];
        let direction = self.step.direction_stream.next();

        self.step.node_id = match direction {
            Direction::Left => &left,
            Direction::Right => &right,
        };
        Some(self.step.clone())
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
