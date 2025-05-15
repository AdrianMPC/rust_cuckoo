use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::traits::keyed::Keyed;

// TO-DO: Make program generate this hashing keys and store them in a file
// can also let the user generate them
pub fn generic_hash<K: Hash>(key: &K, capacity: usize) -> usize {
    // this must be deterministic
    let mut hasher = AHasher::new_with_keys(0xdead_beef_cafe_babe, 0x1337_c0de_f00d_face);
    key.hash(&mut hasher);
    (hasher.finish() as usize) % capacity
}