# lcmx-rs

Calculate the least common multiple of multiple values.

[![Rust](https://github.com/rikonaka/lcmx-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/lcmx-rs/actions/workflows/rust.yml)

## Example

```rust
use lcmx::lcmx;

fn main() {
    let v: Vec<usize> = vec![1, 2, 3, 4];
    let l = lcmx(&v).unwrap();
    println!("{}", l);
    assert_eq!(l, 12);

    let v: Vec<u32> = vec![1, 2, 3, 99];
    let l = lcmx(&v).unwrap();
    println!("{}", l);
    assert_eq!(l, 198);
}
```
