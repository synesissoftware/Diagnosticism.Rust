// diagnostics/mod.rs

use crate::macros::declare_and_publish;

declare_and_publish!(debug_squeezer, DebugSqueezer);
declare_and_publish!(doomgram, DoomGram, doom_scope);
declare_and_publish!(ellipsis, Ellipsis);
mod flf;
declare_and_publish!(password, Password);
declare_and_publish!(pub
    time_format,
    NanosecondsStr,
    nanoseconds_to_string,
);


// ///////////////////////////// end of file //////////////////////////// //
