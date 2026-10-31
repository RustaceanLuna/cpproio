A lightweight library providing IO functionalities for competitive programming (CP)

# Usage
## Example

```rust
use cpproio::*;
use std::io;

fn main() {
    let stdin = FullInput::new(io::stdin().lock());

    input! {
        n: usize,
        mut a: (usize, usize),
        s: String,
        mut grid: Vec<bool>[n; n],
    }

    /* solve the problem here */
}
```

## Features
- Supports different input sources, like `stdin` or files
- Input is read directly from the source as bytes - (mostly) no UTF-8 validation overhead
- A wide variety of types can be parsed (see `input!` docs)
- Works well with [`cargo-equip`](https://github.com/qryxip/cargo-equip)

## Input Sources
This crate provides 2 different input sources, both implementing the `InputSource` trait:
- `FullInput`: Reads all input at once. Use this for most problems, since it's faster by
minimizing system calls and context switches
- `LineInput`: Reads input line by line. Use this for interactive problems, where each
input depends on the previous output

## Error handling
Bad input will panic instead of returning a `Result`, as bad input in competitive
programming is almost always a bug, and errors are usually not handled.
