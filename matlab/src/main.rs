use std::io::{self, Write};

mod colours;
use colours::Colours;

mod matrix;
mod matlab;

fn main() {
	let stdin = io::stdin();
	let mut stdout = io::stdout();

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
				std::process::exit(1);
			}
		}

		matlab::evaluate(&input);
	}
}
