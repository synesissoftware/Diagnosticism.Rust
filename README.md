# Diagnosticism.Rust <!-- omit in toc -->

Diagnosticism, for Rust

[![Crates.io](https://img.shields.io/crates/v/diagnosticism.svg)](https://crates.io/crates/Diagnosticism.Rust)


## Introduction

**Diagnosticism** is a library providing miscellaneous discrete and simple diagnostics facilities to supplement what is available in the standard library. It implemented in several languages, providing enhancements that are necessary (and possible). For example, [**Diagnosticism.Python**](https://github.com/synesissoftware/Diagnosticism.Python) can provide the `trace()` function that can capture the callstack information to issue into a diagnostic log statement.

In **Rust**, which has many powerful reflective and diagnostic elements built in, the facilities are (currently) aimed around supplementing the `std::fmt::Debug` trait. For example, the `Ellipsis` component is able to be used in a custom  `Debug` implementation for a given type to elide certain aspects of the instance's state that is not of interest in the logs or, perhaps, to elide with `"{:?}"` but still include in `"{:#?}"` (the `#alternate()` form). (See the [Examples](#examples) section below for more on this.)

Other facilities (that are not directly related to `Debug`) will be added to the crate soon (and will be listed in the [Components](#components) section below).


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)
	- [Constants](#constants)
	- [Enumerations](#enumerations)
	- [Features](#features)
	- [Functions](#functions)
	- [Macros](#macros)
	- [Structures](#structures)
	- [Traits](#traits)
- [Examples](#examples)
	- [Example - `DoomGram`](#example---doomgram)
	- [Example - `Ellipsis`](#example---ellipsis)
- [Project Information](#project-information)
	- [Where to get help](#where-to-get-help)
	- [Contribution guidelines](#contribution-guidelines)
	- [Dependencies](#dependencies)
		- [Dev Dependencies](#dev-dependencies)
	- [Related projects](#related-projects)
	- [License](#license)


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
diagnosticism = { version = "0" }
```


## Components

### Constants

No public constants are defined at this time.


### Enumerations

No public enumerations are defined at this time.


### Features

No public crate-specific features are defined at this time.


### Functions

No public functions are defined at this time.


### Macros

No public macros are defined at this time.


### Structures

The following structures are defined:

* `DebugSqueezer` - used to assist with restricting the length of `Debug` forms of fields within a given width. See the example [**examples/debug_squeezer.md**](./examples/debug_squeezer.md);
* `DoomGram` - a **D**ecimal **O**rder-**O**f-**M**agnitude histo**G**ram structure that records efficiently duration values in the orders of magnitude 1ns+, 10ns+, 100ns+, 1µs+, ..., 10s+, 100s+ and provides a mechanism for displaying this histogram in a simple single 12-character display, which is useful for logging cumulative execution costs of components in long-running performance-sensitive applications. See the example [**examples/doomgram.md**](./examples/doomgram.md);
* `Ellipsis` - provides strings such as `"********"` to be used for fields that are sensitive and whose `Debug` forms are not to be expressed. See the example [**examples/ellipsis.md**](./examples/ellipsis.md);
* `Password` - Simple type that provides the string `"..."` to be used for fields whose `Debug` forms are not to be expressed. See the example [**examples/password.md**](./examples/password.md);


### Traits

No public traits are defined at this time.


## Examples

Examples are provided in the ```examples``` directory, along with a markdown description for each. A detailed list TOC of them is provided in [EXAMPLES.md](./EXAMPLES.md).


### Example - `DoomGram`

The example program **doomgram** (in **examples** directory, built with feature `test-doomgram`), illustrates use of `DoomGram` to capture the order-of-magnitude histogram of a large number of small random delays. The program source is:

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

and a typical output is:

```plaintext
`#to_strip()` : _aabdedba___ (in 1.763µs)

dg=DoomGram {
    event_count: 20000,
    event_time_total: 12881133711,
    has_overflowed: false,
    min_event_time: Some(
        59,
    ),
    max_event_time: Some(
        197384416,
    ),
    num_events_in_1ns: 0,
    num_events_in_10ns: 3,
    num_events_in_100ns: 7,
    num_events_in_1us: 56,
    num_events_in_10us: 3000,
    num_events_in_100us: 13222,
    num_events_in_1ms: 3687,
    num_events_in_10ms: 23,
    num_events_in_100ms: 2,
    num_events_in_1s: 0,
    num_events_in_10s: 0,
    num_events_ge_100s: 0,
}
```

showing the exploded `Debug` form of the `DoomGram` instance and its timing strip that, for particular execution, obtains the value `"_aabdedba___"` that indicates that there have been:
- 0 events in the 1ns+, 1s+, 10s+, 100s+ magnitudes;
- 1-9 events in 10ns+, 100ns+, 100ms+ magnitudes;
- 10-99 events in 1µs+, 10ms+ magnitudes;
- 1000-9999 events in 10µs+, 10ms+ magnitudes;
- 10000-99999 events in the 100µs+ magnitude;

Naturally, in a live system one would not be employing the exploded `Debug` view, relying only on the terse and efficient timing strip format.


### Example - `Ellipsis`

In the example program **ellipsis** (in **examples** directory), the following types are defined to illustrate the benefit of using `Ellipsis` to provide concise `Debug` output in terse (i.e. non-`#alternate()`) form:

```Rust
/// Large structure that provides the internals of `Thing`
#[derive(Clone)]
#[derive(Debug)]
struct LargeInternal {
    f1 : i64,
    f2 : f32,
    f3 : BTreeMap<&'static str, &'static str>,
    f4 : BTreeSet<u64>,
    f5 : Vec<HashMap<BTreeMap<i32, String>, BTreeSet<u64>>>,
}

#[derive(Debug)]
struct Thing1 {
    name : String,
    internals : Option<LargeInternal>,
}

struct Thing2 {
    name : String,
    internals : Option<LargeInternal>,
}

impl std::fmt::Debug for Thing2 {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        if f.alternate() {
            f
                .debug_struct("Thing2")
                .field("name", &self.name)
                .field("internals", &self.internals)
                .finish()
        } else {
            let ellipsis = Ellipsis::default();

            f
                .debug_struct("Thing2")
                .field("name", &self.name)
                .field("internals", &ellipsis)
                .finish()
        }
    }
}
```

and the program's output illustrates these differing forms, as shown in this filtered form:

```plaintext
Terse `Debug` form of `thing1`: Thing1 { name: "i-am-a-public-thing", internals: Some(LargeInternal { f1: -123, f2: -3.4028235e38, f3: {"a": "A", "b": "B", "c": "C", "d": "D", "e": "E", "f": "F"}, f4: {1, 2, 4, 11, 12345678}, f5: [] }) }

Terse `Debug` form of `thing2`: Thing2 { name: "i-am-a-public-thing", internals: ... }
```


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/Diagnosticism.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/Diagnosticism.Rust.


### Dependencies

**Diagnosticism.Rust** has no (non-development) dependencies.

#### Dev Dependencies

Crates upon which **Diagnosticism.Rust** has development dependencies:

* [**criterion**](https://github.com/bheisler/criterion.rs);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);


### Related projects

* [**Diagnosticism.Python**](https://github.com/synesissoftware/Diagnosticism.Python);


### License

**Diagnosticism.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->

