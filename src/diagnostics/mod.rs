// diagnostics/mod.rs

macro_rules! declare_and_publish {
    ($mod_name:ident, $trait_name:ident) => {
        mod $mod_name;
        pub use $mod_name::$trait_name;
    };
}

declare_and_publish!(ellipsis, Ellipsis);


// ///////////////////////////// end of file //////////////////////////// //
