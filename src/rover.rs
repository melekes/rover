use anyhow::Result;

use std::collections::{BTreeMap, HashMap};

/// Column's index (0...255)
pub type ColumnIndex = u8;

/// ValueDecoder transforms a value into a vector of Columns.
pub trait ValueDecoder<V>
where
    V: AsRef<[u8]>,
{
    fn decode(&self, value: V) -> Vec<Column>;
}

/// Column can either be a i32 or a String.
pub enum Column {
    Number(i32),
    Str(String),
}

/// Rover is an inmemory indexer, which can be used to index any KV database. A `value_decoder` is
/// used to transform a value into a vector of Columns. Then, for each column, a HashMap and
/// BTreeMap are built. A hashmap gives O(1) access, a btree map gives us sorted list.
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

    pub fn index_all_columns(&self, k: K, v: V) -> Result<()> {
        let columns = self.value_decoder.decode(v);
        for c in &columns {
            self.index_column(k.as_ref(), c)?;
        }
        Ok(())
    }

    fn index_column(&self, k: &[u8], c: &Column) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::borsh::BorshValueDecoder;

    struct SingleStringValueDecoder {}
    impl<V> ValueDecoder<V> for SingleStringValueDecoder
    where
        V: AsRef<[u8]>,
    {
        fn decode(&self, v: V) -> Vec<Column> {
            let mut columns = Vec::new();
            let s = String::from_utf8(v.as_ref().to_vec()).unwrap();
            columns.push(Column::Str(s));
            columns
        }
    }

    #[test]
    fn it_indexes_all_columns() {
        let r: Rover<&[u8], &[u8]> = Rover::new(Box::new(BorshValueDecoder {}));
        for (k, v) in [(1, "a"), (2, "b"), (3, "c")] {
            r.index_all_columns(&(k as i32).to_be_bytes(), v.as_bytes());
        }
    }
}
