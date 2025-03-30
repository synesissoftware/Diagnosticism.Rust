# Diagnosticism.Rust Example - **doomgram**

## Summary

An example using **Diagnosticism.Rust**'s `DoomGram` type to represent the performance of some time-consuming operations.


## Source

```Rust
// examples/doomgram.rs : example program illustrating use of `DoomGram`

use diagnosticism::diagnostics::DoomGram;

use rand::{
    rngs::StdRng,
    RngCore,
    SeedableRng,
};

use std::{
    thread,
    time::{
        Duration,
        Instant,
    },
};


fn main() {
    let mut rng = <StdRng as SeedableRng>::seed_from_u64(123456789);

    let mut dg = DoomGram::default();

    // do a warmup loop first
    for w in 0..2 {
        for i in 0..20000 {
            let mut v = rng.next_u32() % 1000000;

            if 0 == i % 10 {
                if 0 != i {
                    v = v % i;
                }
            }

            let before = Instant::now();

            if 0 != i % 2000 {
                thread::sleep(Duration::from_nanos(v as u64));
            } else {
                // no wait, so should be very low ns

                thread::sleep(Duration::from_secs(0));
            }

            let after = Instant::now();

            dg.push_event_duration(after - before);
        }

        // output results on second run through
        if 1 == w {
            let before = Instant::now();
            let strip = dg.to_strip();
            let after = Instant::now();

            eprintln!("`#to_strip()` : {strip} (in {:?})", after - before);
            eprintln!("");
            eprintln!("dg={dg:#?}");
        }

        dg.clear();
    }
}
```


## Running and output

When executed, as in:

```bash
$ cargo run --example doomgram --features test-doomgram
```

it gives the output:

```
`#to_strip()` : _aacdeda____ (in 18.765µs)

dg=DoomGram {
    event_count: 20000,
    event_time_total: 11864811133,
    has_overflowed: false,
    min_event_time: Some(
        66,
    ),
    max_event_time: Some(
        13556334,
    ),
    num_events_in_1ns: 0,
    num_events_in_10ns: 5,
    num_events_in_100ns: 5,
    num_events_in_1us: 143,
    num_events_in_10us: 2971,
    num_events_in_100us: 13060,
    num_events_in_1ms: 3813,
    num_events_in_10ms: 3,
    num_events_in_100ms: 0,
    num_events_in_1s: 0,
    num_events_in_10s: 0,
    num_events_ge_100s: 0,
}
```


<!-- ########################### end of file ########################### -->

