# Diagnosticism.Rust - TODO <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [Functional improvements](#functional-improvements)
- [Performance improvements](#performance-improvements)


## Functional improvements

* [x] ~~~simple file/line/function macros~~~:
  * [x] `fileline!()`;
  * [x] `function!()`;
  * [x] `filelinefunction!()`;
* [ ] advanced file/line/function macros, using procedural macros, such that, say, `function!()` acts as a literal (and can participate in `concat!()`);


## Performance improvements

* [ ] `DebugSqueezer`: avoid formatting the entire [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) string before truncation (current implementation uses `format!()` then `truncate()`; investigate streaming/early-limit approaches that preserve readable elision suffixes without allocating for oversized output);


<!-- ########################### end of file ########################### -->

