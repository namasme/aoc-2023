use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

pub trait Dijkstra<Node, Distance> {
    fn distance(&self, seeds: &[Node], is_goal: impl Fn(&Node) -> bool) -> Option<Distance>;
}

pub trait Graph<Node, Distance> {
    fn neighbours(&self, node: &Node) -> Vec<(Distance, Node)>;
}

#[derive(Debug, Eq, PartialEq)]
struct DijkstraDistanceNode<Node, Distance> {
    node: Node,
    cumulative_distance: Distance,
}

impl<Node, Distance> DijkstraDistanceNode<Node, Distance> {
    fn from(cumulative_distance: Distance, node: Node) -> DijkstraDistanceNode<Node, Distance> {
        DijkstraDistanceNode {
            cumulative_distance,
            node,
        }
    }
}

impl<Node, Distance> Ord for DijkstraDistanceNode<Node, Distance>
where
    Node: Eq + PartialEq,
    Distance: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.cumulative_distance.cmp(&self.cumulative_distance)
    }
}

impl<Node, Distance> PartialOrd for DijkstraDistanceNode<Node, Distance>
where
    Node: Eq + PartialEq,
    Distance: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Node, Distance, T> Dijkstra<Node, Distance> for T
where
    T: Graph<Node, Distance>,
    Node: Copy + Eq + Hash + PartialEq,
    Distance: Add<Output = Distance> + Copy + Default + Ord + PartialOrd,
{
    fn distance(&self, seeds: &[Node], is_goal: impl Fn(&Node) -> bool) -> Option<Distance> {
        let mut cumulative_distances: HashMap<Node, Distance> = HashMap::new();
        let mut unvisited: BinaryHeap<DijkstraDistanceNode<Node, Distance>> = BinaryHeap::new();

        for seed in seeds.iter().cloned() {
            unvisited.push(DijkstraDistanceNode::from(Default::default(), seed));
            cumulative_distances.insert(seed, Default::default());
        }

        while let Some(current) = unvisited.pop() {
            if is_goal(&current.node) {
                return Some(current.cumulative_distance);
            }

            let best_cumulative_distance = cumulative_distances[&current.node];

            if best_cumulative_distance < current.cumulative_distance {
                continue;
            }
            // TODO what happens if i take two simulatenous entries?
            for (edge_distance, neighbour) in self.neighbours(&current.node) {
                let entry = cumulative_distances.entry(neighbour);
                let candidate_cumulative_distance = current.cumulative_distance + edge_distance;
                if let Entry::Occupied(mut o) = entry {
                    let best_yet = o.get_mut();
                    if *best_yet > candidate_cumulative_distance {
                        *best_yet = candidate_cumulative_distance;
                        unvisited.push(DijkstraDistanceNode::from(
                            candidate_cumulative_distance,
                            neighbour,
                        ));
                    }
                } else {
                    entry.or_insert(candidate_cumulative_distance);
                    unvisited.push(DijkstraDistanceNode::from(
                        candidate_cumulative_distance,
                        neighbour,
                    ));
                }
            }
        }
        None
    }
}
