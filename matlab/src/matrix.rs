pub struct Matrix {
	rows: usize,
	columns: usize,

	buffer: Box<[i64]>
}

impl Matrix {
	pub fn new(rows: usize, columns: usize) -> Self {
		let buffer = vec![0; rows * columns];
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
}
