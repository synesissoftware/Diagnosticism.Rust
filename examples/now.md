# Diagnosticism.Rust - Example - **now**

## Summary

An example using **Diagnosticism.Rust**'s `now()` function to obtain a UTC timestamp string.


## Source

```rust
use diagnosticism::diagnostics::now;

fn main() {
    let now = now();

    println!("now: {now}");
}
```


## Running and output

When executed, as in:

```bash
$ cargo run --example now
```

it prints one line beginning with `now: `, followed by the current UTC time.


<!-- ########################### end of file ########################### -->
