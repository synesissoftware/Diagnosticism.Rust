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
    }
}

/// T.B.C.
#[macro_export]
macro_rules! function_fully_qualified_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
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

/// T.B.C.
#[macro_export]
macro_rules! function_name_only {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
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
    }}
}

/// T.B.C.
#[macro_export]
macro_rules! filelinefunction_fully_qualified_name {
    () => {{
        format!("{}:{}", $crate::fileline!(), $crate::function_fully_qualified_name!())
    }}
}


/// T.B.C.
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
