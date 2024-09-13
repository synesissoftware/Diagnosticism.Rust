// diagnostics/password.rs : `Password`

use std::fmt as std_fmt;


const NUM_SPLATS_DEFAULT : usize = 8;

const SPLATS_LITERAL : &str = "****************************************************************************************************";


#[derive(Default)]
pub struct Password {
	num_splats : Option<usize>,
}

impl Password {
	pub fn new(num_splats : usize) -> Self {
		let num_splats = Some(num_splats);

		Self {
			num_splats,
		}
	}
}

impl std_fmt::Debug for Password {
	fn fmt(
		&self,
		f: &mut std_fmt::Formatter<'_>,
	) -> std_fmt::Result {

		let num_splats = self.num_splats.unwrap_or(NUM_SPLATS_DEFAULT);

		if num_splats > SPLATS_LITERAL.len() {
			let splats = String::from_utf8(vec![b'*'; num_splats]).unwrap();

			f.write_str(&splats)
		} else {

			f.write_str(&SPLATS_LITERAL[0..num_splats])
		}
	}
}


#[cfg(test)]
mod tests {
	#![allow(non_snake_case)]

	use super::{
		Password,
		SPLATS_LITERAL,
	};


	#[test]
	fn TEST_Password_DEFAULT_num_splats() {
		let password = Password::default();

		let expected = "********";
		let actual = format!("{password:?}");

		assert_eq!(expected, actual);
	}

	#[test]
	fn TEST_Password_80_SPLATS() {
		let password = Password::new(80);

		let expected = &SPLATS_LITERAL[0..80];
		let actual = format!("{password:?}");

		assert_eq!(80, actual.len());

		assert_eq!(expected, actual);
	}

	#[test]
	fn TEST_Password_100_SPLATS() {
		let password = Password::new(100);

		let expected = &SPLATS_LITERAL[0..100];
		let actual = format!("{password:?}");

		assert_eq!(100, actual.len());

		assert_eq!(expected, actual);
	}

	#[test]
	fn TEST_Password_200_SPLATS() {
		let password = Password::new(200);

		let actual = format!("{password:?}");

		assert_eq!(200, actual.len());
        assert!(actual.chars().all(|c| c == '*'));
	}
}


// ///////////////////////////// end of file //////////////////////////// //

