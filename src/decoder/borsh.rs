use crate::rover::{Column, ValueDecoder as RoverValueDecoder};

pub struct ValueDecoder {}

impl<V> RoverValueDecoder<V> for ValueDecoder
where
    V: AsRef<[u8]>,
{
    fn decode(&self, _v: V) -> Vec<Column> {
        panic!("unimplemented")
    }
}
