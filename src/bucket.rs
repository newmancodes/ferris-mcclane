use std::fmt::{Display, Formatter, Error};

pub struct Bucket {
    id: usize,
    capacity: u8,
    used_capacity: u8,
}

impl Bucket {
    pub fn new(id: usize, capacity: u8, used_capacity: u8) -> Result<Self, String> {
        if capacity == 0 {
            return Err(String::from("Buckets can not have zero capacity."));
        }

        if used_capacity > capacity {
            return Err(format!("A bucket with capacity {} can not have used {} of that capacity", capacity, used_capacity));
        }

        Ok(Bucket {
            id,
            capacity,
            used_capacity,
        })
    }

    pub fn empty(id: usize, capacity: u8) -> Result<Self, String> {
        Bucket::new(id, capacity, 0)
    }

    pub fn full(id: usize, capacity: u8) -> Result<Self, String> {
        Bucket::new(id, capacity, capacity)
    }

    pub fn is_empty(&self) -> bool {
        self.used_capacity == 0
    }

    pub fn remaining_capacity(&self) -> u8 {
        self.capacity - self.used_capacity
    }

    pub fn is_full(&self) -> bool {
        self.remaining_capacity() == 0
    }
}

impl Display for Bucket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(id: {}, {}/{})", self.id, self.used_capacity, self.capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_buckets_can_be_created() {
        let desired_capacity: u8 = 56;
        let bucket = Bucket::empty(12,56).unwrap();
        assert_eq!(bucket.is_empty(), true);
        assert_eq!(bucket.remaining_capacity(), desired_capacity);
        assert_eq!(bucket.is_full(), false);
    }

    #[test]
    fn empty_buckets_can_not_have_a_zero_capacity() {
        let bucket = Bucket::empty(87, 0);
        assert!(bucket.is_err());
    }

    #[test]
    fn full_buckets_can_be_created() {
        let bucket = Bucket::full(9, 56).unwrap();
        assert_eq!(bucket.is_empty(), false);
        assert_eq!(bucket.remaining_capacity(), 0);
        assert_eq!(bucket.is_full(), true);
    }

    #[test]
    fn full_buckets_can_not_have_a_zero_capacity() {
        let bucket = Bucket::empty(6, 0);
        assert!(bucket.is_err());
    }

    #[test]
    fn partially_full_buckets_can_be_created() {
        let bucket = Bucket::new(12, 56, 38).unwrap();
        assert_eq!(bucket.is_empty(), false);
        assert_eq!(bucket.remaining_capacity(), 18);
        assert_eq!(bucket.is_full(), false);
    }

    #[test]
    fn partially_buckets_can_not_have_a_zero_capacity() {
        let bucket = Bucket::new(76, 0, 0);
        assert!(bucket.is_err());
    }

    #[test]
    fn buckets_can_not_be_overfilled() {
        let bucket = Bucket::new(87, 56, 57);
        assert!(bucket.is_err());
    }
}