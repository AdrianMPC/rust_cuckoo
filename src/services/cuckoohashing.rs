use crate::utils::generic_hash;
use std::ops::Mul;

pub struct CuckooHashTable<T> {
    table: Vec<Option<T>>,
    capacity: usize,
}

impl<T: Keyed + Eq + Copy> CuckooHashTable<T> {
    pub fn new(capacity: usize) -> Result<Self, TryReserveError> {
        let table = Vec::try_with_capacity(capacity)?.into_iter()
            .map(|_| None)
            .collect();
        Ok(Self { table, capacity })
    }

    fn first_hash(&self, item: &T) -> usize {
        generic_hash(item.key(), self.capacity)
    }

    fn second_hash(&self, item: &T) -> usize {
        let key = item.key_for_second_hash();
        generic_hash(&*key, self.capacity)
    }


}
