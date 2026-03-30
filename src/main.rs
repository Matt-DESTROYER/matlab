use std::io::{stdin, stdout, Write};

mod tools;

mod colours;
use colours::println_error;

mod matrix;

mod tokeniser;

mod parser;

mod matlab;
use crate::matlab::Evaluator;

fn main() {
	let stdin = stdin();
	let mut stdout = stdout();

	let mut evaluator = Evaluator::new();

	loop {
		print!(">> ");
		match stdout.flush() {
			Ok(_) => {},
			Err(_) => {}
		};

		let mut input = String::new();
		match stdin.read_line(&mut input) {
			Ok(_) => {},
			Err(_) => {
				println_error("Error: Could not read input...".to_owned());
				continue;
			}
		}
		input = input.trim().to_string();

		if &input.to_lowercase() == "quit" {
			return;
		}

		evaluator.evaluate(&input);
	}
}
