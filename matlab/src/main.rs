use std::io::{self, Write};

mod tools;
mod colours;
use colours::Colours;
mod matrix;
mod tokeniser;
mod matlab;
use crate::matlab::Evaluator;

fn main() {
	let stdin = io::stdin();
	let mut stdout = io::stdout();

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
				eprintln!("{}Error: Could not read input...{}", Colours::RED, Colours::RESET);
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
