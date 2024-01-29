use aoc_2023::day8;
use num::integer::lcm;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day8/input").unwrap();
    let (directions, network) = day8::parse_input(&input);

    // So the textbook solution here would be to identify the loop length (lambda)
    // for each seed and the (position of) the first occurrence of a terminal node within
    // the loop (T), then solve the system of equations n ≡ T_i (mod lambda_i) applying CRT.
    //
    // At least for my input, however, the gcd of the lambda_is is > 1 (269), so CRT cannot
    // be applied straight away. The equations can be melded together by replacing
    // (lambda_i, lambda_j) with lcm(lambda_i, lambda_j) and (T_i, T_j) with a suitable T.
    // In my case though T_i == lambda_i for every i, which means it's 0 for the purposes
    // of the congruences.
    //
    // The whole set of equations then degenerates into finding the lcm for all the cycle lengths.

    let cycle_lengths: Vec<_> = network
        .edges
        .keys()
        .filter(|node_id| is_initial(node_id))
        .map(|node_id| network.detect_cycle(node_id, &directions).lambda)
        .collect();
    let answer = cycle_lengths.into_iter().reduce(lcm).unwrap();
    println!("{answer}");
}

fn is_initial(node_id: &day8::NodeID) -> bool {
    node_id.ends_with('A')
}

// Keeping the implementation around just for the sake of illustrating the
// approach, and because we all succumb to the sunk cost phallacy anyway.

// fn find_first_terminal(
//     network: &day8::Network,
//     seed: &day8::NodeID,
//     directions: &[day8::Direction],
// ) -> usize {
//     let step = day8::Step {
//         node_id: seed,
//         network,
//         direction_stream: day8::DirectionStream::from_directions(directions),
//     };
//     let mu = network.detect_cycle(seed, directions).mu;
//     mu + step
//         .into_iter()
//         .skip(mu)
//         .take_while(|current| !is_terminal(current.node_id))
//         .count()
// }

// fn is_terminal(node_id: &day8::NodeID) -> bool {
//     node_id.ends_with('Z')
// }
