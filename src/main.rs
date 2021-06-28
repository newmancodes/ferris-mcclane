mod bucket;
mod bucket_puzzle;
mod iterative_deepening_solver;

use bucket::Bucket;
use bucket_puzzle::{ BucketPuzzle, Rules };
use iterative_deepening_solver::IterativeDeepeningSolver;

fn main() {
    let can_fill = true;
    let can_empty = true;

    let buckets: Vec<Bucket> = vec!(
        Bucket::empty(0, 3).unwrap(),
        Bucket::empty(1, 5).unwrap(),
    );

    let target_capacity: u8 = 4;

    let rules = Rules::new(can_fill, can_empty);
    let initial_state = BucketPuzzle::from(buckets, &rules, target_capacity);
    let search_depth_limit = 3;
    let solver = IterativeDeepeningSolver::new(initial_state, search_depth_limit);
    match solver.solve() {
        Some(solution) => println!("Sweet found a solution {}.", solution),
        None => println!("No solution found with search depth limit of {}.", search_depth_limit),
    }
/*
    println!("Initial State:\n{}\n", initial_state);

    let mut explored = Vec::new();
    let mut frontier = vec!(initial_state);

    for child_state in puzzle.expand(5) {
        println!("Child State:\n{}\n", child_state);
    }
 */
}
