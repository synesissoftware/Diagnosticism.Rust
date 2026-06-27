//! Simple diagnostics utilities for Rust — part of the cross-language
//! **Diagnosticism** family.
//!
//! **Diagnosticism** offers small, focused helpers that extend the
//! standard library for logging, profiling, and debug output. The
//! project is implemented in several languages; each port exposes
//! facilities that are useful and idiomatic in that environment.
//! (See [**Diagnosticism.Python**][dp] for a wider API, including
//! tracing and callstack capture.)
//!
//! In Rust, this crate focuses on three areas:
//!
//! * **[`Debug`](std::fmt::Debug) helpers** — control what appears in
//!   log output ([`Ellipsis`], [`Password`], [`DebugSqueezer`]);
//! * **Timing** — record duration distributions ([`DoomGram`]),
//!   measure closures ([`doom_scope`]), and format durations via
//!   [`nanoseconds_to_string`] into [`NanosecondsStr`];
//! * **Source location** — compile-time file, line, and function
//!   macros (`fileline!`, `filelinefunction!`, and others).
//!
//! For example, [`Ellipsis`] in a custom [`Debug`](std::fmt::Debug)
//! implementation can elide fields in terse `"{:?}"` output while
//! still including them in alternate `"{:#?}"` form.
//!
//! [dp]: https://github.com/synesissoftware/Diagnosticism.Python
//!
//! # Installation
//!
//! Reference in **Cargo.toml** in the usual way:
//!
//! ```toml
//! diagnosticism = { version = "0" }
//! ```
//!
//! # Components
//!
//! ## Types and functions
//!
//! The following are re-exported at the crate root (and also available in
//! [`diagnostics`]):
//!
//! * [`DebugSqueezer`] — restrict the length of
//!   [`Debug`](std::fmt::Debug) output for individual fields;
//! * [`DoomGram`] — decimal order-of-magnitude histogram with a compact
//!   12-character strip for logging;
//! * [`Ellipsis`] — emit `"..."` for redacted
//!   [`Debug`](std::fmt::Debug) fields;
//! * [`Password`] — emit a run of `*` characters for sensitive
//!   [`Debug`](std::fmt::Debug) fields;
//! * [`doom_scope`] — time a closure and record the elapsed duration in a
//!   [`DoomGram`];
//! * [`NanosecondsStr`] — compact storage for a formatted duration string;
//! * [`nanoseconds_to_string`] — format a nanosecond count into a
//!   [`NanosecondsStr`];
//!
//! ## Macros (crate root)
//!
//! File, line, and function helpers are exported at the crate root:
//!
//! * [`fileline!`] — file name and line number at the call site;
//! * [`filelinefunction!`] — file, line, and unqualified function name;
//! * [`filelinefunction_fully_qualified_name!`] — file, line, and
//!   fully-qualified function name;
//! * [`function_fully_qualified_name!`] — fully-qualified name of the
//!   enclosing function;
//! * [`function_name_only!`] — unqualified name of the enclosing function;
//! * [`type_name_only!`] — unqualified name of a given type;
//!
//! ## Redacting [`Debug`](std::fmt::Debug) fields
//!
//! Both [`Ellipsis`] and [`Password`] are field placeholders in custom
//! [`Debug`](std::fmt::Debug) implementations; neither reads the underlying
//! value.
//!
//! * [`Ellipsis`] — `"..."` for verbose, non-sensitive elision; often used
//!   with `{:#?}` alternate output;
//! * [`Password`] — a masked `*` run for sensitive values; use
//!   [`Password::new`] for width;
//!
//! # Examples
//!
//! ```
//! use diagnosticism::Ellipsis;
//!
//! println!("redacted: {:?}", Ellipsis::default());
//! ```
//!
//! Further examples are provided in the repository **examples** directory
//! and in the project
//! [README](https://github.com/synesissoftware/Diagnosticism.Rust).

// lib.rs

pub mod diagnostics;

pub use diagnostics::{
    doom_scope,
    nanoseconds_to_string,
    DebugSqueezer,
    DoomGram,
    Ellipsis,
    NanosecondsStr,
    Password,
};


// ///////////////////////////// end of file //////////////////////////// //
