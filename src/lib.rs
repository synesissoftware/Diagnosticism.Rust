//! Miscellaneous discrete and simple diagnostics facilities for Rust.
//!
//! **Diagnosticism** supplements what is available in the standard library.
//! It is implemented in several languages; in Rust the facilities are
//! (currently) aimed around supplementing [`Debug`](std::fmt::Debug),
//! together with lightweight timing and source-location helpers.
//!
//! For example, [`Ellipsis`](diagnostics::Ellipsis) can be used in a custom
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
//! ## Types and functions ([`diagnostics`])
//!
//! * [`DebugSqueezer`](diagnostics::DebugSqueezer) — restrict the length of
//!   [`Debug`](std::fmt::Debug) output for individual fields;
//! * [`DoomGram`](diagnostics::DoomGram) — decimal order-of-magnitude
//!   histogram with a compact 12-character strip for logging;
//! * [`Ellipsis`](diagnostics::Ellipsis) — emit `"..."` for redacted
//!   [`Debug`](std::fmt::Debug) fields;
//! * [`Password`](diagnostics::Password) — emit a run of `*` characters for
//!   sensitive [`Debug`](std::fmt::Debug) fields;
//! * [`doom_scope`](diagnostics::doom_scope) — time a closure and record
//!   the elapsed duration in a [`DoomGram`](diagnostics::DoomGram);
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
//! # Examples
//!
//! ```
//! use diagnosticism::diagnostics::Ellipsis;
//!
//! println!("redacted: {:?}", Ellipsis::default());
//! ```
//!
//! Further examples are provided in the repository **examples** directory
//! and in the project
//! [README](https://github.com/synesissoftware/Diagnosticism.Rust).

// lib.rs

pub mod diagnostics;


// ///////////////////////////// end of file //////////////////////////// //
