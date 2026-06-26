// src/diagnostics/ellipsis.rs : `Ellipsis`

use std::fmt as std_fmt;


/// Placeholder for [`Debug`](std::fmt::Debug) output that prints `"..."`.
///
/// Use [`Ellipsis`] to omit verbose or low-value field detail from logs
/// while keeping the field name visible. It suits custom
/// [`Debug`](std::fmt::Debug) implementations that show full detail only in
/// alternate form (`{:#?}`).
///
/// For sensitive values (credentials, tokens, secrets), prefer
/// [`Password`](crate::diagnostics::Password), which prints a masked run of
/// `*` characters instead.
#[derive(Default)]
pub struct Ellipsis {}

impl std_fmt::Debug for Ellipsis {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        write!(f, "...")
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::Ellipsis;


    #[test]
    fn TEST_Ellipsis_Debug() {
        let ellipsis = Ellipsis::default();

        assert_eq!("...", format!("{ellipsis:?}"));
    }
}


// ///////////////////////////// end of file //////////////////////////// //
