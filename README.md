# incremental-average-rs

Support for incremental averaging, allowing to add and subtract numbers in an average as you go.

[![Continuous integration](https://github.com/ismaelJimenez/incremental-average-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/ismaelJimenez/incremental-average-rs/actions/workflows/rust.yml)

## Usage

To use `incremental-average`, add this to your `Cargo.toml`:

```toml
[dependencies]
incremental-average = "0.1.0"
```

## Examples

Add/Remove new elements to the average.

```rust
let mut ia = IncrementalAverage::new();

assert!(ia.get().is_none());

ia.add(1.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 1.0);

ia.add(1.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 1.0);

ia.add(4.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 2.0);

ia.add(0.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 1.5);

ia.remove(0.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 2.0);

ia.remove(4.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 1.0);

ia.remove(1.0);
ia.get();

assert!(ia.get().is_some());
assert_eq!(ia.get().unwrap(), 1.0);
```
