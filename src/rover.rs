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
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
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

    pub fn index_all_columns(&mut self, k: K, v: V) {
        let columns = self.value_decoder.decode(v);
        for (i, c) in columns.into_iter().enumerate() {
            // XXX: possible overflow
            self.index_column(k, c, i as u8);
        }
    }

    fn index_column(&mut self, k: K, c: Column, index: ColumnIndex) {
        let c_copy = c.clone();
        // hashmap
        match self.maps.get_mut(&index) {
            Some(m) => match m.get_mut(&c) {
                Some(keys) => keys.push(k),
                None => {
                    m.insert(c, vec![k]);
                }
            },

            None => {
                let mut m = HashMap::new();
                m.insert(c, vec![k]);
                self.maps.insert(index, m);
            }
        }

        // btreemap
        match self.btrees.get_mut(&index) {
            Some(m) => match m.get_mut(&c_copy) {
                Some(keys) => keys.push(k),
                None => {
                    m.insert(c_copy, vec![k]);
                }
            },

            None => {
                let mut m = BTreeMap::new();
                m.insert(c_copy, vec![k]);
                self.btrees.insert(index, m);
            }
        }
    }

    /// Returns a vector of keys or None if no keys are associated with the given Column.
    pub fn get(&self, c: Column, index: ColumnIndex) -> Option<&Vec<K>> {
        self.maps.get(&index).and_then(|m| m.get(&c))
    }

    /// Returns a vector of keys sorted by the given column. Note keys with the same column are in
    /// order which they were indexed.
    pub fn sort_by_column(&self, index: ColumnIndex) -> Vec<K> {
        self.btrees.get(&index).map_or(Vec::new(), |m| {
            m.values().fold(Vec::new(), |mut acc, x| {
                acc.append(x.clone().as_mut());
                acc
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Column is a single String.
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
        let mut r: Rover<&str, &str> = Rover::new(Box::new(SingleStringValueDecoder {}));
        for (k, v) in [("1", "a"), ("2", "b"), ("3", "c")] {
            r.index_all_columns(k, v);
        }

        assert_eq!(Some(&vec!["1"]), r.get(Column::Str("a".to_string()), 0));
    }

    #[test]
    fn sort_by_column_returns_correct_order() {
        let mut r: Rover<&str, &str> = Rover::new(Box::new(SingleStringValueDecoder {}));
        for (k, v) in [("1", "b"), ("2", "a"), ("3", "c")] {
            r.index_all_columns(k, v);
        }
        assert_eq!(vec!["2", "1", "3"], r.sort_by_column(0));
    }
}
