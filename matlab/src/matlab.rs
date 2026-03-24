use crate::matrix::Matrix;
use std::str::FromStr;

pub fn evaluate(input: &str) {
	println!("{}", input);
}

impl FromStr for Matrix {
	type Err = String;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		Ok(Matrix::new(2, 2))
	}
}
