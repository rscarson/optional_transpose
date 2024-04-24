# optional_transpose
A small crate that adds `.transpose()` to `Option<Option<T>>`
It allows you to reversibly swap the inner and outer options in the pair, so that you can use `?` on the inner option

[![Crates.io](https://img.shields.io/crates/v/optional_transpose.svg)](https://crates.io/crates/optional_transpose)

Example:
```rust
use optional_transpose::OptionTranspose;

fn example() -> Option<i32> {
    let x: Option<Option<i32>> = Some(None);
    let y: Option<i32> = x.transpose()?; // Returns None, as the inner option is None
    Some(1)
}

assert_eq!(example(), None);
```