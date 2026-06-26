// src/diagnostics/flf.rs : Defines file+line+function macros

/// Expands to the file name and line number in which it was invoked.
///
/// As with the standard [`file!`], [`line!`], and [`column!`] macros, these
/// macros provide debugging information for developers about location
/// within the source.
///
/// # Examples
///
/// ```
/// let this_file_line = diagnosticism::fileline!();
/// println!("defined at this file/line: {this_file_line}");
/// ```
#[macro_export]
macro_rules! fileline {
    () => {
        concat!(file!(), ":", line!())
    };
}

/// Expands to the fully-qualified name of the function in which it was
/// invoked.
///
/// The name is obtained via [`std::any::type_name`] of a nested helper at
/// the call site. It evaluates to a `&'static str` at runtime and is not a
/// compile-time string literal (unlike [`fileline!`]).
///
/// # Examples
///
/// ```
/// fn my_function() {
///     let name = diagnosticism::function_fully_qualified_name!();
///     println!("inside {name}");
/// }
/// ```
#[macro_export]
macro_rules! function_fully_qualified_name {
    () => {{
        fn f() {
        }
        fn type_name_of<T>(_ : T) -> &'static str {
            std::any::type_name::<T>()
        }

        let name : &'static str = type_name_of(f);

        let name = if name.ends_with("::f") {
            &name[..name.len() - 3]
        } else {
            name
        };

        name
    }};
}

/// Expands to the unqualified name of the function in which it was invoked.
///
/// This is the same path as [`function_fully_qualified_name!`], truncated
/// at the last path separator (`::`). It evaluates to a `&'static str` at
/// runtime.
///
/// # Examples
///
/// ```
/// fn my_function() {
///     let name = diagnosticism::function_name_only!();
///     println!("called from {name}");
/// }
/// ```
#[macro_export]
macro_rules! function_name_only {
    () => {{
        fn f() {
        }
        fn type_name_of<T>(_ : T) -> &'static str {
            std::any::type_name::<T>()
        }

        let name : &'static str = type_name_of(f);

        let name = if name.ends_with("::f") {
            &name[..name.len() - 3]
        } else {
            name
        };

        match &name[..name.len()].rfind(':') {
            Some(pos) => &name[pos + 1..name.len()],
            None => &name[..name.len()],
        }
    }};
}

/// Expands to the file name and line number and function name in which it
/// was invoked.
///
/// As with the standard [`file!`], [`line!`], and [`column!`] macros, these
/// macros provide debugging information for developers about location
/// within the source.
///
/// Unlike [`fileline!`], this macro allocates a [`String`] via [`format!`]
/// because the function name is obtained at runtime from
/// [`function_name_only!`].
///
/// # Examples
///
/// ```
/// let this_file_line_fn = diagnosticism::filelinefunction!();
/// println!("defined at this file/line/function: {this_file_line_fn}");
/// ```
#[macro_export]
macro_rules! filelinefunction {
    () => {{
        format!("{}:{}", $crate::fileline!(), $crate::function_name_only!())
    }};
}

/// Expands to the file name, line number, and fully-qualified function name
/// at the call site.
///
/// Unlike [`fileline!`], this macro allocates a [`String`] via [`format!`]
/// because it combines a compile-time file/line prefix with a runtime
/// function name from [`function_fully_qualified_name!`].
///
/// # Examples
///
/// ```
/// let loc = diagnosticism::filelinefunction_fully_qualified_name!();
/// println!("at {loc}");
/// ```
#[macro_export]
macro_rules! filelinefunction_fully_qualified_name {
    () => {{
        format!("{}:{}", $crate::fileline!(), $crate::function_fully_qualified_name!())
    }};
}


/// Expands to the unqualified [`std::any::type_name`] of the given type.
///
/// The `$type_name` argument is a type token (e.g. `String`, `Self`, or a
/// path to a struct). Generic substitutions are not applied; the result
/// reflects the type as written at the call site.
///
/// # Examples
///
/// ```
/// struct Widget;
///
/// assert_eq!("Widget", diagnosticism::type_name_only!(Widget));
/// ```
#[macro_export]
macro_rules! type_name_only {
    ($type_name:tt) => {{
        let name : &'static str = std::any::type_name::<$type_name>();

        match &name[..name.len()].rfind(':') {
            Some(pos) => &name[pos + 1..name.len()],
            None => &name[..name.len()],
        }
    }};
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    struct SomeCustomType {}

    impl SomeCustomType {
        fn indirect_name() -> &'static str {
            type_name_only!(Self)
        }
    }


    #[test]
    fn TEST_fileline() {
        let expected = format!("{}:{}", file!(), (line!() + 1));
        let actual = fileline!();

        assert_eq!(expected, actual);
    }

    #[test]
    fn TEST_filelinefunction() {
        let expected = format!("{}:{}:{}", file!(), (line!() + 1), "TEST_filelinefunction");
        let actual = filelinefunction!();

        assert_eq!(expected, actual);
    }

    #[test]
    fn TEST_type_name_only_WITH_SomeCustomType() {
        {
            let expected = "SomeCustomType";
            let actual = type_name_only!(SomeCustomType);

            assert_eq!(expected, actual);
        }

        {
            let expected = "SomeCustomType";
            let actual = SomeCustomType::indirect_name();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn TEST_type_name_only_WITH_STANDARD_TYPES() {
        {
            let expected = "String";
            let actual = type_name_only!(String);

            assert_eq!(expected, actual);
        }

        {
            let expected = "i32";
            let actual = type_name_only!(i32);

            assert_eq!(expected, actual);
        }
    }
}
