use thiserror::Error;

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
#[derive(Eq, PartialEq, Hash)]
pub enum Column {
    Number(i32),
    Str(String),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid rdo_lookahead_frames {0} (expected < {})", i32::MAX)]
    InvalidLookahead(u32),
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
    maps: HashMap<ColumnIndex, HashMap<Column, Vec<K>>>,
    // iterating over sorted keys
    btrees: HashMap<ColumnIndex, BTreeMap<Column, Vec<K>>>,
    // a decoder which knows how to transform raw bytes into a vector of Column
    value_decoder: Box<dyn ValueDecoder<V> + 'static>,
}

impl<K, V> Rover<K, V>
where
    K: AsRef<[u8]> + Copy,
    V: AsRef<[u8]>,
{
    pub fn new(value_decoder: Box<dyn ValueDecoder<V>>) -> Self {
        Self {
            maps: HashMap::new(),
            btrees: HashMap::new(),
            value_decoder,
        }
    }

    pub fn index_all_columns(&mut self, k: K, v: V) -> Result<(), Error> {
        let columns = self.value_decoder.decode(v);
        let mut i: u8 = 0;
        for c in columns {
            self.index_column(k, c, i)?;
            i += 1; // XXX: possible overflow
        }
        Ok(())
    }

    fn index_column(&mut self, k: K, c: Column, index: ColumnIndex) -> Result<(), Error> {
        // hashmap
        match self.maps.get_mut(&index) {
            Some(m) => match m.get_mut(&c) {
                Some(keys) => keys.push(k),
                None => {
                    m.insert(c, vec![k]);
                    ()
                }
            },

            None => {
                let mut m = HashMap::new();
                m.insert(c, vec![k]);
                self.maps.insert(index, m);
            }
        }

        // btreemap
        // TODO

        Ok(())
    }

    /// Returns a vector of keys or None if no keys are associated with the given Column.
    pub fn get(&self, c: Column, index: ColumnIndex) -> Option<&Vec<K>> {
        self.maps.get(&index).map_or(None, |m| m.get(&c))
    }

    pub fn sort_by(c: Column) -> Vec<K> {
        panic!("unimplemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut r: Rover<&[u8], &[u8]> = Rover::new(Box::new(SingleStringValueDecoder {}));
        for (k, v) in [("1", "a"), ("2", "b"), ("3", "c")] {
            r.index_all_columns(k.as_bytes(), v.as_bytes()).unwrap();
        }

        assert_eq!(
            Some(vec!["1".as_bytes()].as_ref()),
            r.get(Column::Str("a".to_string()), 0)
        );
    }
}
