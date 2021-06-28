use super::bucket::Bucket;

use std::fmt::{Display, Formatter, Error};

pub struct BucketPuzzle<'a> {
    buckets: &'a [Bucket],
    rules: &'a Rules,
    target_volume: u8,
    depth: u8,
}

#[derive(Debug)]
pub struct Rules {
    can_fill: bool,
    can_empty: bool,
}

impl<'a> BucketPuzzle<'a> {
    pub fn from(buckets: &'a [Bucket], rules: &'a Rules, target_volume: u8) -> Self {
        BucketPuzzle {
            buckets,
            rules,
            target_volume,
            depth: 0,
        }
    }

    pub fn expand(&'a self, limit: u8) -> Option<Vec<&BucketPuzzle<'a>>> {
        if self.depth >= limit {
            return None;
        }

        let mut expanded: Vec<&'a BucketPuzzle> = Vec::new();
        Some(expanded)
    }
}

impl Display for BucketPuzzle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Rules: {:?}", self.rules);
        write!(f, "Target Volume: {}", self.target_volume);
        for bucket in self.buckets {
            write!(f, "{}", bucket);
        };
        Ok(())
    }
}

impl Rules {
    pub fn new(can_fill: bool, can_empty: bool) -> Self {
        Rules {
            can_fill,
            can_empty,
        }
    }
}