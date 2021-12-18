use std::collections::{BTreeMap, HashMap};

// There can't be more than 255 columns.
pub type ColumnIndex = u8;

pub trait ValueDecoder<V>
where
    V: AsRef<[u8]>,
{
    fn decode(&self, value: V) -> Vec<Column>;
}

// Column can either be a i32 or a String.
pub enum Column {
    Number(i32),
    Str(String),
}

pub struct Rover<K, V>
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    // O(1) access (hard requirement)
    maps: HashMap<ColumnIndex, HashMap<K, V>>,
    // iterating over sorted keys
    btrees: HashMap<ColumnIndex, BTreeMap<K, V>>,
    // a decoder which knows how to transform raw bytes into a vector of Column
    value_decoder: Box<dyn ValueDecoder<V> + 'static>,
}

impl<K, V> Rover<K, V>
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    pub fn new(value_decoder: Box<dyn ValueDecoder<V>>) -> Self {
        Self {
            maps: HashMap::new(),
            btrees: HashMap::new(),
            value_decoder,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_indexes_all_keys() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(2, "b");
        map.insert(1, "a");

        // let value_decoder = gg
        let mut r = Rover::new(value_decoder);
    }
}
