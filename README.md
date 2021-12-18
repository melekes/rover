# Rover

An indexer for any KV database.

```toml
[dependencies]
rover = "0.0.1"
```

## Example

```rust
let mut r: Rover<&[u8], &[u8]> = Rover::new(Box::new(SingleStringValueDecoder {}));
for (k, v) in [("1", "a"), ("2", "b"), ("3", "c")] {
    r.index_all_columns(k.as_bytes(), v.as_bytes());
}

assert_eq!(
    Some(vec!["1".as_bytes()].as_ref()),
    r.get(Column::Str("a".to_string()), 0)
);
```


