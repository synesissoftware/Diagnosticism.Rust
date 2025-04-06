// src/diagnostics/logging.rs : miscellaneous logging utility functions

use chrono;


pub fn now() -> String {
    let now = chrono::Utc::now();

    now.format("%H:%m:%S%.6f").to_string().into()
}

