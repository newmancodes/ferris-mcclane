use super::bucket::Bucket;

use std::fmt::{Display, Formatter, Error};

pub struct BucketPuzzle<'a> {
    buckets: Vec<Bucket>,
    rules: &'a Rules,
    target_volume: u8,
    depth: u8,
    parent: Option<&'a BucketPuzzle<'a>>,
    reason: Option<String>,
}

pub struct Rules {
    can_fill: bool,
    can_empty: bool,
}

impl<'a> BucketPuzzle<'a> {
    pub fn from(buckets: Vec<Bucket>, rules: &'a Rules, target_volume: u8) -> Self {
        BucketPuzzle {
            buckets,
            rules,
            target_volume,
            depth: 0,
            parent: None,
            reason: None,
        }
    }

    fn from_parent(parent: &'a BucketPuzzle, buckets: Vec<Bucket>, reason: String) -> Self {
        BucketPuzzle {
            buckets,
            rules: parent.rules,
            target_volume: parent.target_volume,
            depth: parent.depth + 1,
            parent: Some(parent),
            reason: Some(reason),
        }
    }

    pub fn is_goal_state(&'a self) -> bool {
        for bucket in &self.buckets {
            if bucket.used_capacity() == self.target_volume {
                return true;
            }
        }

        false
    }

    pub fn expand(&'a self, limit: u8) -> Vec<BucketPuzzle<'a>> {
        let mut expanded: Vec<BucketPuzzle<'a>> = Vec::new();

        if self.depth < limit {
            for bucket in &self.buckets {
                if self.rules.can_empty && !bucket.is_empty() {
                    expanded.push(self.empty(&bucket));
                }

                if self.rules.can_fill && !bucket.is_full() {
                    expanded.push(self.fill(&bucket));
                }

                if !bucket.is_empty() {
                    for other_bucket in &self.buckets {
                        if bucket != other_bucket
                            && !other_bucket.is_full() {
                            expanded.push(self.pour(&bucket, &other_bucket));
                        }
                    }
                }
            }
        }

        expanded
    }

    fn empty(&'a self, bucket: &Bucket) -> BucketPuzzle<'a> {
        let mut buckets: Vec<Bucket> = Vec::with_capacity(self.buckets.len());

        for existing_bucket in &self.buckets {
            if bucket == existing_bucket {
                buckets.push(existing_bucket.pour_all());
            }
            else {
                buckets.push(existing_bucket.clone());
            }
        }

        BucketPuzzle::from_parent(self, buckets, format!("Emptied bucket {}", bucket))
    }

    fn fill(&'a self, bucket: &Bucket) -> BucketPuzzle<'a> {
        let mut buckets: Vec<Bucket> = Vec::with_capacity(self.buckets.len());

        for existing_bucket in &self.buckets {
            if bucket == existing_bucket {
                buckets.push(existing_bucket.fill());
            }
            else {
                buckets.push(existing_bucket.clone());
            }
        }

        BucketPuzzle::from_parent(self, buckets, format!("Filled bucket {}", bucket))
    }

    fn pour(&'a self, from_bucket: &Bucket, to_bucket: &Bucket) -> BucketPuzzle<'a> {
        let mut buckets: Vec<Bucket> = Vec::with_capacity(self.buckets.len());
        let mut pour_amount = from_bucket.used_capacity();
        if to_bucket.remaining_capacity() < pour_amount {
            pour_amount = to_bucket.remaining_capacity();
        }
        let pour_amount = pour_amount;

        for existing_bucket in &self.buckets {
            if from_bucket == existing_bucket {
                buckets.push(existing_bucket.pour_from(pour_amount).unwrap());
            }
            else if to_bucket == existing_bucket {
                buckets.push(existing_bucket.pour_into(pour_amount).unwrap());
            }
            else {
                buckets.push(existing_bucket.clone());
            }
        }

        BucketPuzzle::from_parent(self, buckets, format!("Poured {} from bucket {} into bucket {}", pour_amount, from_bucket, to_bucket))
    }
}

impl Display for BucketPuzzle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Target Volume: {}.", self.target_volume).unwrap();
        writeln!(f, "Rules: {}", self.rules).unwrap();
        match &self.reason {
            Some(reason) => writeln!(f, "Performed: {}.", reason).unwrap(),
            None => {},
        };
        write!(f, "Now: ").unwrap();
        let mut insert_separator = false;
        for bucket in &self.buckets {
            if insert_separator {
                write!(f, ", ").unwrap();
            }

            write!(f, "{}", bucket).unwrap();
            insert_separator = true;
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

impl Display for Rules {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut insert_separator = false;

        if self.can_fill {
            write!(f, "Can fill").unwrap();
            insert_separator = true;
        }

        if self.can_empty {
            if insert_separator {
                write!(f, ". ").unwrap();
            }

            write!(f, "Can empty").unwrap();
            insert_separator = true;
        }

        if insert_separator {
            write!(f, ". ").unwrap();
        }

        Ok(())
    }
}