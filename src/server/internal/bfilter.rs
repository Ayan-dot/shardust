use probabilistic_collections::count_min_sketch::{CountMinSketch, CountMinStrategy};
use probabilistic_collections::cuckoo::CuckooFilter;
use std::{cmp, collections::HashSet, hash::Hash};

pub struct CountFilter<K> {
    t_tize: u64,
    sketches: CountMinSketch<CountMinStrategy, K>,
    bloom_filter: CuckooFilter<K>,
    max_size: u64,
}

impl<K> CountFilter<K>
where
    K: Hash + Eq + Clone,
{
    fn sketch(&self, key: &K) -> i64 {
        return self.sketches.count(key)
            + if self.bloom_filter.contains(key) {
                1
            } else {
                0
            };
    }
}
