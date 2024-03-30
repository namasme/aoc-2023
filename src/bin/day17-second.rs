use aoc_2023::day17;
use aoc_2023::graph::Dijkstra;
use aoc_2023::spatial::UPoint2D;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day17/input").unwrap();
    let city_map = UltracrucibleCityMap(day17::BaseCityMap::parse(&input));
    let width = city_map.0 .0.get_width();
    let height = city_map.0 .0.get_height();
    let goal = UPoint2D::from(height - 1, width - 1);

    let answer = city_map
        .distance(&day17::CruciblePathStep::seeds(), |node| {
            node.position == goal && node.consecutive >= 4
        })
        .unwrap();

    println!("{answer}");
}

pub struct UltracrucibleCityMap(day17::BaseCityMap);

impl day17::CityMap for UltracrucibleCityMap {
    fn get_height(&self) -> usize {
        self.0 .0.get_height()
    }

    fn get_width(&self) -> usize {
        self.0 .0.get_width()
    }

    fn is_valid_step(
        &self,
        current: &day17::CruciblePathStep,
        candidate: &day17::CruciblePathStep,
    ) -> bool {
        if current.direction == candidate.direction {
            candidate.consecutive <= 10
        } else {
            current.consecutive >= 4
        }
    }

    fn heat_loss_at(&self, position: UPoint2D) -> day17::HeatLoss {
        *self.0 .0.at(position)
    }
}
