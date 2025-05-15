use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::traits::keyed::Keyed;

pub fn generic_hash<K: Hash>(key: &K, capacity: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % capacity
}