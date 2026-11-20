# Diagnosticism.Rust - Example - **now**

## Summary

An example using **Diagnosticism.Rust**'s `now()` function to obtain a nanosecond-resolution timestamp.


## Source

```rust
use diagnosticism::Password;

use std::fmt as std_fmt;


struct Credentials {
    username : String,
    #[allow(dead_code)]
    password : String,
}

impl std_fmt::Debug for Credentials {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &Password::default())
            .finish()
    }
}


fn main() {
    let credentials = Credentials {
        username : "JohnSmith@email.com".into(),
        password : "abc123".into(),
    };

    println!("credentials: {credentials:#?}");
}
```


## Running and output

When executed, as in:

```bash
$ cargo run --example password
```

it gives the output:

```
credentials: Credentials {
    username: "JohnSmith@email.com",
    password: ********,
}
```


<!-- ########################### end of file ########################### -->



```rust
use diagnosticism::diagnostics::now;

fn main() {
    let now = now();

    println!("now: {now}");
}
```
