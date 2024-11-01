# Diagnosticism.Rust <!-- omit in toc -->

Diagnosticism, for Rust


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

* `DebugSqueezer` - used to assist with restricting the length of `Debug` forms of fields within a given width;
* `Ellipsis` - provides strings such as `"********"` to be used for fields that are sensitive and whose `Debug` forms are not to be expressed;
* `Password` - Simple type that provides the string `"..."` to be used for fields whose `Debug` forms are not to be expressed;


### Traits

No public traits are defined at this time.


## Examples

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

