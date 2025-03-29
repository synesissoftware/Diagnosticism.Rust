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

        let name : &'static str = unsafe {
            let p = name.as_ptr();

            let u = std::slice::from_raw_parts(p, name.len() - 3);

            std::mem::transmute(u)
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

        let name : &'static str = unsafe {
            let p = name.as_ptr();

            let u = std::slice::from_raw_parts(p, name.len() - 3);

            std::mem::transmute(u)
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


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]


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
}
