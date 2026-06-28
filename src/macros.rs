macro_rules! declare_and_publish_impl_ {
    ($mod_name:ident; $($construct_name:ident),*; $vis:vis) => {
        mod $mod_name;

        $vis use $mod_name::{ $($construct_name),* };
    };
}

/// Declares a private submodule and re-exports selected items from it.
///
/// Use in a **`mod.rs` barrel file** when a directory contains sibling
/// `.rs` implementation files. Each invocation replaces:
///
/// ```rust,ignore
/// mod margin;
/// pub use margin::margin;
/// ```
///
/// with a single macro call. The first argument is the **module name**
/// (matching `name.rs` or `name/mod.rs`); remaining arguments are **item
/// names** (types, functions, constants) to re-export from that module.
///
/// # Setup
///
/// ```rust,ignore
/// use crate::macros::declare_and_publish;
/// ```
///
/// # Forms
///
/// * `declare_and_publish!(mod_name, Item)` — `pub use` (default);
/// * `declare_and_publish!(pub mod_name, Item)` — `pub use` (explicit);
/// * `declare_and_publish!(crate mod_name, Item)` — `pub(crate) use`;
/// * `declare_and_publish!(super mod_name, Item)` — `pub(super) use`;
/// * `declare_and_publish!(self mod_name, Item)` — `pub(self) use`;
/// * `declare_and_publish!(priv mod_name, Item)` — `pub(self) use`
///   (synonym);
/// * `declare_and_publish!($vis mod_name, Item)` — any Rust visibility
///   (e.g. `pub(crate)`, `pub(in crate::api)`).
///
/// Multiple items may be re-exported from one module:
///
/// ```rust,ignore
/// declare_and_publish!(doomgram, DoomGram, doom_scope);
/// ```
///
/// A parent module (including **`lib.rs`**) may aggregate a child barrel:
///
/// ```rust,ignore
/// declare_and_publish!(
///     api,
///     evaluate_scalar_eq_approx,
///     margin,
/// );
/// ```
///
/// # Examples
///
/// Public API surface:
///
/// ```rust,ignore
/// declare_and_publish!(password, Password);
/// declare_and_publish!(
///     time_format,
///     NanosecondsStr,
///     nanoseconds_to_string,
/// );
/// ```
///
/// Crate-internal wiring:
///
/// ```rust,ignore
/// declare_and_publish!(crate compare, compare_foo, compare_bar);
/// ```
///
/// Submodules that need no re-export remain ordinary private declarations:
///
/// ```rust,ignore
/// mod flf;
/// ```
macro_rules! declare_and_publish {
    (crate $mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; pub(crate));
    };
    (pub $mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; pub);
    };
    (priv $mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; pub(self));
    };
    (self $mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; pub(self));
    };
    (super $mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; pub(super));
    };
    ($mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; pub);
    };
    ($vis:vis $mod_name:ident $(, $construct_name:ident)* $(,)?) => {
        $crate::macros::declare_and_publish_impl_!($mod_name; $($construct_name),*; $vis);
    };
}

pub(crate) use declare_and_publish;
pub(crate) use declare_and_publish_impl_;


// ///////////////////////////// end of file //////////////////////////// //
