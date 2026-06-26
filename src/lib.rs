//! Miscellaneous discrete and simple diagnostics facilities for Rust.
//!
//! **Diagnosticism** supplements what is available in the standard library.
//! It is implemented in several languages; in Rust the facilities are
//! (currently) aimed around supplementing [`Debug`](std::fmt::Debug),
//! together with lightweight timing and source-location helpers.
//!
//! For example, [`Ellipsis`] can be used in a custom
//! [`Debug`](std::fmt::Debug) implementation to elide fields in terse
//! (`"{:?}"`) output while still including them in alternate (`"{:#?}"`)
//! form.
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
    DebugSqueezer,
    DoomGram,
    Ellipsis,
    Password,
};


// ///////////////////////////// end of file //////////////////////////// //
