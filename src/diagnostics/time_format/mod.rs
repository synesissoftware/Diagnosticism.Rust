// src/diagnostics/time_format/mod.rs : duration formatting

// NOTE: this work was brought in from **asynkio** via **Diagnosticism.Python**
// 0.16.0

mod format;
mod nanoseconds_str;

pub use format::nanoseconds_to_string;
pub use nanoseconds_str::NanosecondsStr;


// ///////////////////////////// end of file //////////////////////////// //
