use crate::rover::{Column, ValueDecoder};

pub struct BorshValueDecoder {}

impl<V> ValueDecoder<V> for BorshValueDecoder
where
    V: AsRef<[u8]>,
{
    fn decode(&self, value: V) -> Vec<Column> {
        Vec::new()
    }
}
