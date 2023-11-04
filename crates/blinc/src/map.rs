use std::{fmt::Debug, hash::Hash};

use ahash::RandomState;
use hashbrown::HashMap;
use parking_lot::RwLock;

pub struct ConcurrentMap<K, V> {
    shards: Box<[RwLock<HashMap<K, V, RandomState>>]>,
    num_shards: usize,
    hasher: RandomState,
}

impl<K: Eq + Hash, V: Clone> Default for ConcurrentMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Debug + Clone + Eq + Hash, V: Debug + Clone> Debug for ConcurrentMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug_map = self
            .shards
            .iter()
            .flat_map(|shard| shard.read().clone())
            .map(|(k, v)| (k, v))
            .collect::<HashMap<_, _>>();

        f.debug_map().entries(debug_map.iter()).finish()
    }
}

impl<K: Eq + Hash, V: Clone> ConcurrentMap<K, V> {
    pub fn new() -> Self {
        let num_shards =
            (std::thread::available_parallelism().map_or(1, usize::from) * 4).next_power_of_two();

        Self {
            shards: (0..num_shards)
                .map(|_| RwLock::new(HashMap::default()))
                .collect::<Box<_>>(),
            num_shards,
            hasher: RandomState::default(),
        }
    }

    fn hash(&self, key: &K) -> usize {
        self.hasher.hash_one(key) as usize
    }

    fn determine_shard(&self, hash: usize) -> usize {
        hash % self.num_shards
    }

    unsafe fn get_read_shard(
        &self,
        idx: usize,
    ) -> parking_lot::lock_api::RwLockReadGuard<parking_lot::RawRwLock, HashMap<K, V, RandomState>>
    {
        self.shards.get_unchecked(idx).read()
    }

    unsafe fn get_write_shard(
        &self,
        idx: usize,
    ) -> parking_lot::lock_api::RwLockWriteGuard<parking_lot::RawRwLock, HashMap<K, V, RandomState>>
    {
        self.shards.get_unchecked(idx).write()
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let idx = self.determine_shard(hash);

        let shard = unsafe { self.get_read_shard(idx) };

        shard.get(key).cloned()
    }

    pub fn get_or_insert<F: FnOnce() -> V>(&self, key: K, value: F) -> V {
        let hash = self.hash(&key);
        let idx = self.determine_shard(hash);

        // First, read the shard with just a read-lock.
        let result = {
            let shard = unsafe { self.get_read_shard(idx) };
            shard.get(&key).cloned()
        };

        // If the result is some, return it.
        if let Some(result) = result {
            return result;
        }

        // Getting the value failed with a read lock, so we will try with a write-lock.
        let mut shard = unsafe { self.get_write_shard(idx) };
        let result = shard.get(&key);

        // We check that the result is some, this means another thread won and wrote first.
        if let Some(result) = result {
            return result.clone();
        }

        // If this thread won, we get the value and insert it.
        let result = value();
        shard.insert(key, result.clone());
        result
    }
}
