use super::bucket_puzzle::BucketPuzzle;

pub struct IterativeDeepeningSolver<'a> {
    initial_state: BucketPuzzle<'a>,
    search_depth_limit: u8,
}

impl<'a> IterativeDeepeningSolver<'a> {
    pub fn new(initial_state: BucketPuzzle<'a>, search_depth_limit: u8) -> Self {
        Self {
            initial_state,
            search_depth_limit,
        }
    }

    pub fn solve(self) -> Option<&'a BucketPuzzle<'a>> {
        let mut explored: Vec<BucketPuzzle<'a>> = Vec::new();
        let mut frontier: Vec<BucketPuzzle<'a>> = vec!(self.initial_state);

        loop {
            match frontier.pop() {
                Some(to_explore) => {
                    if to_explore.is_goal_state() {
                        return Some(to_explore);
                    }
                    else {
                        for child_state in to_explore.expand(self.search_depth_limit) {
                            frontier.push(child_state);
                        }

                        explored.push(to_explore);
                    }
                },
                None => return None,
            }
        }
    }
}