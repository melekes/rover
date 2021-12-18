# Rover

[<img alt="build status" src="https://img.shields.io/github/workflow/status/melekes/rover/CI/master?style=for-the-badge" height="20">](https://github.com/melekes/rover/actions?query=branch%3Amaster)

An indexer for any KV database.

```toml
[dependencies]
rover = "0.1.0"
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

## TODO

- [ ] borsh decoder
- [ ] quickcheck tests
- [ ] fuzz tests

## Details

- Rover is generic over keys and values as long as they can be cheaply
  converted into `&[u8]`.
- When constructing a rover, a decoder argument must be provided, which job is
  to parse values into vectors of `Column`.
- `Column` can either be a number or a string. Is there a way to avoid
  introducing a `Column` struct?
- Rover constructs a `HashMap` and `BTreeMap` for each column. `HashMap` can be
  removed if/when `O(1)` requirement is dropped. Then the time to get keys
  becomes logarithmic (still pretty fast & saves a lot of memory).
