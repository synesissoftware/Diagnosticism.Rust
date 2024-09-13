// src/diagnostics/ellipsis.rs : `Ellipsis`

use std::fmt as std_fmt;


/// Simple type that provides the string `"..."` to be used for fields whose
/// `Debug` forms are not to be expressed.
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
