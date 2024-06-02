# lcmx-rs

Calculate the least common multiple of multiple values.

[![Rust](https://github.com/rikonaka/lcmx-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/lcmx-rs/actions/workflows/rust.yml)

## Example

```rust
use lcmx::lcmx;

fn main() {
    let v = vec![1, 2, 3, 4];
    let l = lcmx(&v).unwrap();
    println!("{}", l);
    assert_eq!(l, 12);
}
```
