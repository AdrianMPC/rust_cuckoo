use crate::utils::generic_hash;
use crate::models::response::ResponseLayout;
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

    fn get_table_size(&self){
        return self.table.len();
    }

    fn check_existence(&self, id:usize) -> bool {
        let index_pos1 : usize = first_hash(&id);

        if self.table[index_pos1].id == id {
            return true;
        }

        let index_pos2 = : usize = second_hash(&id);
        if self.table[index_pos2].id == id {
            return true;
        }

        return false; 
    }

    fn find_data(&self, id:usize) -> &ResponseLayout {
        let index_pos1 : usize = first_hash(&id);

        if self.table[index_pos1].id == id {
            return self.table[index_pos1];
        }

        let index_pos2 = : usize = second_hash(&id);
        if self.table[index_pos2].id == id {
            return self.table[index_pos2];
        }

        return None; 
    }
}
