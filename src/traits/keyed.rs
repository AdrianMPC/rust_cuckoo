use std::hash::Hash;
use std::ops::Mul;
use crate::models::keytypes::{IntegerKey,StringKey};

pub trait Keyed {
    type Key: Hash + Eq;
    fn key(&self) -> &Self::Key;
    fn key_for_second_hash(&self) -> Box<dyn Hash>;
}

impl<K> Keyed for IntegerKey<K>
where
    K: Hash + Copy + Mul<Output = K> + From<u8>,
{
    type Key = K;

    fn key(&self) -> &Self::Key {
        &self.id
    }

    fn key_for_second_hash(&self) -> Box<dyn Hash> {
        let offset = K::from(31u8);
        let salted = self.id * offset;
        Box::new(salted)
    }
}

impl Keyed for StringKey {
    type Key = String;

    fn key(&self) -> &Self::Key {
        &self.name
    }

    fn key_for_second_hash(&self) -> Box<dyn Hash> {
        let salted = format!("{}#offset", self.name);
        Box::new(salted)
    }
}