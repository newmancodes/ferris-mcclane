mod bucket;

use bucket::Bucket;

fn main() {
    let can_fill = true;
    let can_empty = true;

    let buckets: Vec<Bucket> = vec!(
        Bucket::empty(3).expect(""),
        Bucket::full(5).expect(""),
    );

    let target_capacity: u8 = 4;


}
