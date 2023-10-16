# 52-weeks-rust
Trying out Rust

## Trying to learn Rust

**Quick Example**
```rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
``````

- [Learn By Example!](https://doc.rust-lang.org/rust-by-example/)