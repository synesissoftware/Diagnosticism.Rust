# Diagnosticism.Rust - CHANGES <!-- omit in toc -->


## 0.4.2 - 14th July 2026

* expanded unit-tests;
* application of underscores in number literals;
* general tidying;
* updated dependencies;
* clippy issues;


## 0.4.1 - 7th July 2026

* implement `Len` for `NanosecondsStr`;


## 0.4.0 - 30th June 2026

* added `DoomGram::to_mmm()` and `DoomGram::to_nmmm()` — compact min/mean/max duration summaries using `nanoseconds_to_string()`;


## 0.3.2 - 28th June 2026

* optimisation of `nanoseconds_to_string()` — uses a custom return type `NanosecondsStr` for highly efficient conversion in vast majority of cases;


## 0.3.1 - 28th June 2026

* internal implementation improvements;


## 0.3.0 - 28th June 2026

* added `nanoseconds_to_string()` — compact human-readable duration formatting (behaviour matches **Diagnosticism.Python** 0.16.0);


## 0.2.1 - 27th June 2026

* added CI tasks;
* fixing Clippy issues (all simple);
* minor improvements to `DoomGram` public method(s);
* cargo fmt;
* comment consistency;
* add documentation markup;
* Cargo.toml crate attributes;
* removed dead code in `DebugSqueezer` truncation branch;
* removed README.md invalid claim to be dependent on **test_help-rs**;
* improved documentation of `Ellipsis` and `Password`;
* updated **README.md** with all current constructs;


## 0.2.0 - 24th July 2025

* added `doom_scope<>()`;


## 0.1.0 - 29th March 2025

* added file+line+function macros (initial implementations);


## 0.0.2 - 19th December 2024

* added `DoomGram`;


## 0.0.1 - 13th September 2024

* added `DebugSqueezer`;
* added `Ellipsis`;
* added `Password`;


## 0.0.0 - 27th August 2024

* initial (empty) version;


All history before this day is moot!


<!-- ########################### end of file ########################### -->

