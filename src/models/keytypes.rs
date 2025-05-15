#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerKey<K> {
    pub id: K,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKey {
    pub name: String,
}