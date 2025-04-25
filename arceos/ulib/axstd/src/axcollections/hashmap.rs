extern crate alloc;
use super::hash::DefaultHasher;
use alloc::vec;
use alloc::vec::Vec;
use core::clone::Clone;
use core::hash::{Hash, Hasher};

/// A dumb HashMap
pub struct HashMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
    size: usize,
    current_index: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: vec![None; 16],
            size: 0,
            current_index: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.size >= self.buckets.len() {
            self.resize();
        }
        let index = Self::hash(&key, self.buckets.len());
        self.buckets[index] = Some((key, value));
        self.size += 1;
    }

    fn hash(key: &K, bucket_len: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % bucket_len
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 3;
        let mut new_buckets = vec![None; new_capacity];
        for bucket in &mut self.buckets.drain(..) {
            if let Some((key, value)) = bucket {
                let index = Self::hash(&key, new_capacity);
                new_buckets[index] = Some((key, value));
            }
        }
        self.buckets = new_buckets;
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.buckets.iter().filter_map(|bucket| bucket.as_ref())
    }
}

impl<K: Hash + Eq, V> Iterator for HashMap<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.buckets.len() {
            if let Some((key, value)) = self.buckets[self.current_index].take() {
                self.current_index += 1;
                self.size -= 1;
                return Some((key, value));
            }
            self.current_index += 1;
        }
        None
    }
}
