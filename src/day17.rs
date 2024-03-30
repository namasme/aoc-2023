use crate::common::Matrix;
use crate::graph;
use crate::spatial::Direction;
use crate::spatial::Orientation;
use crate::spatial::UPoint2D;

pub type HeatLoss = u64;

pub struct BaseCityMap(pub Matrix<HeatLoss>);

impl BaseCityMap {
    pub fn parse(input: &str) -> Self {
        let data = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as HeatLoss)
            .collect();
        let width = input.lines().next().unwrap().len();
        Self(Matrix { data, width })
    }
}

pub trait CityMap {
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;
    fn is_valid_step(&self, current: &CruciblePathStep, candidate: &CruciblePathStep) -> bool;
    fn heat_loss_at(&self, position: UPoint2D) -> HeatLoss;
}

impl<CityMapT> graph::Graph<CruciblePathStep, HeatLoss> for CityMapT
where
    CityMapT: CityMap,
{
    fn neighbours(&self, node: &CruciblePathStep) -> Vec<(HeatLoss, CruciblePathStep)> {
        node.neighbours()
            .into_iter()
            .filter(|neighbour| {
                neighbour
                    .position
                    .within_bounds(self.get_width(), self.get_height())
                    && self.is_valid_step(node, neighbour)
            })
            .map(|neighbour| (self.heat_loss_at(neighbour.position), neighbour))
            .collect()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct CruciblePathStep {
    pub position: UPoint2D,
    pub direction: Direction,
    pub consecutive: u8,
}

impl CruciblePathStep {
    pub fn seeds() -> Vec<Self> {
        vec![
            CruciblePathStep {
                position: UPoint2D::from(0, 0),
                direction: Direction::Right,
                consecutive: 0,
            },
            CruciblePathStep {
                position: UPoint2D::from(0, 0),
                direction: Direction::Down,
                consecutive: 0,
            },
        ]
    }

    fn neighbours(&self) -> Vec<Self> {
        let left_turn = self.direction.rotate(Orientation::Counterclockwise);
        let right_turn = self.direction.rotate(Orientation::Clockwise);
        let build_candidate = |direction, consecutive| {
            let new_position = self.position.move_by(direction);

            new_position.map(|position| Self {
                position,
                direction,
                consecutive,
            })
        };
        let candidates = vec![
            build_candidate(self.direction, self.consecutive + 1),
            build_candidate(left_turn, 1),
            build_candidate(right_turn, 1),
        ];

        candidates
            .into_iter()
            // position is valid
            .flatten()
            .collect()
    }
}
