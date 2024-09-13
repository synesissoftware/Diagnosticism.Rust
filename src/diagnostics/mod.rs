// diagnostics/mod.rs

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),*) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(debug_squeezer, DebugSqueezer);
declare_and_publish!(ellipsis, Ellipsis);
declare_and_publish!(password, Password);


// ///////////////////////////// end of file //////////////////////////// //
