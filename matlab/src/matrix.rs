use std::ops::{Add, Sub, Mul};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
	rows: usize,
	columns: usize,

	buffer: Box<[f64]>
}

#[allow(dead_code)]
impl Matrix {
	pub fn new(rows: usize, columns: usize) -> Self {
		let buffer = vec![0.0; rows * columns];
		Self {
			rows: rows,
			columns: columns,
			buffer: buffer.into_boxed_slice()
		}
	}

	pub fn rows(&self) -> usize {
		self.rows
	}
	pub fn columns(&self) -> usize {
		self.columns
	}

	pub fn get(&self, row: usize, column: usize) -> Result<f64, &str> {
		if row >= self.rows || column >= self.columns {
			return Err("Out of range");
		}
		Ok(self.buffer[row * self.columns + column])
	}
	pub fn at(&self, row: usize, column: usize) -> Result<f64, &str> {
		self.get(row, column)
	}
	pub fn set(&mut self, value: f64, row: usize, column: usize) -> Result<(), &str> {
		if row >= self.rows || column >= self.columns {
			return Err("Out of range");
		}
		self.buffer[row * self.columns + column] = value;

		Ok(())
	}

	pub fn checked_add(&mut self, other: &Self) -> Result<&Self, &str> {
		if self.rows != other.rows || self.columns != other.columns {
			return Err("Matrix dimensions don't match");
		}

		for i in 0..self.buffer.len() {
			self.buffer[i] += other.buffer[i];
		}

		Ok(self)
	}
	pub fn add_scalar(&mut self, num: f64) {
		for i in 0..self.buffer.len() {
			self.buffer[i] += num;
		}
	}

	pub fn checked_subtract(&mut self, other: &Self) -> Result<&Self, &str> {
		if self.rows != other.rows || self.columns != other.columns {
			return Err("Matrix dimensions don't match");
		}

		for i in 0..self.buffer.len() {
			self.buffer[i] -= other.buffer[i];
		}

		Ok(self)
	}
	pub fn checked_sub(&mut self, other: &Self) -> Result<&Self, &str> {
		self.checked_subtract(other)
	}
	pub fn subtract_scalar(&mut self, num: f64) {
		self.add_scalar(-num);
	}

	pub fn checked_multiply(&self, other: &Self) -> Result<&Self, &str> {
		if self.columns != other.rows {
			return Err("Cannot multiply matrices of incompatible dimensions");
		}

		let mut result = Matrix::new(self.rows, other.columns);

		// each element of the result array
		for row in 0..self.rows {
			for column in 0..other.columns {
				// iterate through rows of first array and columns of second array
				for i in 0..self.columns {
					result.buffer[row * result.columns + column] += self.buffer[row * self.columns + i] * other.buffer[i * other.columns + column];
				}
			}
		}
		

		Ok(self)
	}
	pub fn checked_mult(&self, other: &Self) -> Result<&Self, &str> {
		self.checked_multiply(other)
	}
	pub fn checked_mul(&self, other: &Self) -> Result<&Self, &str> {
		self.checked_multiply(other)
	}
	pub fn multiply_by_scalar(&mut self, num: f64) {
		for i in 0..self.buffer.len() {
			self.buffer[i] *= num;
		}
	}
	pub fn divide_by_scalar(&mut self, num: f64) {
		for i in 0..self.buffer.len() {
			self.buffer[i] /= num;
		}
	}

	pub fn to_string(&self) -> String {
		let mut stringified = String::new();
		stringified.push('[');
		for row in 0..self.rows() {
			stringified.push('\n');
			stringified.push_str("  ");
			for column in 0..self.columns() {
				stringified.push_str(&format!("{} ", self.get(row, column).unwrap()));
			}
		}
		if self.rows > 0 {
			stringified.push('\n');
		}
		stringified.push(']');

		stringified
	}
}

impl Add for Matrix {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.checked_add(&rhs)
			.expect("Matrix addition failed");
		self
	}
}

impl Sub for Matrix {
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self.checked_subtract(&rhs)
			.expect("Matrix subtraction failed");
		self
	}
}

impl Mul for Matrix {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		self
			.checked_multiply(&rhs)
			.expect("Matrix multiplication failed");
		self
	}
}

impl Display for Matrix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}
