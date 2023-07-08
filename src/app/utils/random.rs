use std::ops::{Range, RangeInclusive};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Random {
    rng: SmallRng,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed)
        }
    }

    pub fn range<T: SampleUniform + PartialOrd>(&mut self, range: Range<T>) -> T
    {
        if range.start < range.end { self.rng.gen_range(range) } else { range.start }
    }

    pub fn range_inclusive<T: SampleUniform + PartialOrd>(&mut self, range: RangeInclusive<T>) -> T
    {
        if range.start() < range.end() {
            self.rng.gen_range(range)
        } else {
            let (start, _) = range.into_inner();
            start
        }
    }
}