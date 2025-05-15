#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericKey<K> {
    pub id: K,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKey {
    pub name: String,
}