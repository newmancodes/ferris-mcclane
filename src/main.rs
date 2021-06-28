mod bucket;
mod bucket_puzzle;

use bucket::Bucket;
use bucket_puzzle::{ BucketPuzzle, Rules };

fn main() {
    let can_fill = true;
    let can_empty = true;

    let mut buckets: Vec<Bucket> = vec!(
        Bucket::as_empty(0, 3).unwrap(),
        Bucket::as_full(1, 5).unwrap(),
    );

    let target_capacity: u8 = 4;

    let rules = Rules::new(can_fill, can_empty);
    let puzzle = BucketPuzzle::from(buckets, &rules, target_capacity);
    println!("{}\n", puzzle);
    for child_state in puzzle.expand(5) {
        println!("{}\n", child_state);
    }
}
