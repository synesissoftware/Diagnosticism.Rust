// diagnostics/mod.rs

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(debug_squeezer, DebugSqueezer);
declare_and_publish!(doomgram, DoomGram, doom_scope);
declare_and_publish!(ellipsis, Ellipsis);
mod flf;
declare_and_publish!(password, Password);
declare_and_publish!(time_format, nanoseconds_to_string);


// ///////////////////////////// end of file //////////////////////////// //
