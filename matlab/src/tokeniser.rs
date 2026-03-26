use crate::matrix::Matrix;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Operator {
	Power,
	Add,
	Subtract,
	Multiply,
	Divide,
	Not,
	EqualTo,
	NotEqualTo,
	LessThan,
	LessThanOrEqualTo,
	GreaterThan,
	GreaterThanOrEqualTo,
	Assign
}

#[derive(Clone)]
pub enum Token {
	Number(f64),
	Variable(String),
	Operator(Operator),
	Matrix(Matrix)
}

#[derive(Copy, Clone, PartialEq)]
enum TokenType {
	None,
	Number,
	Variable,
	Operator,
	Matrix
}

fn is_number(char: char) -> bool {
	char.is_ascii_digit() || char == '.'
}
fn is_matrix(char: char) -> bool {
	char == '['
}
fn is_operator(char: char) -> bool {
	"+-*/<>!=".contains(char)
}
fn is_variable(char: char) -> bool {
	char.is_alphabetic()
}

fn parse_operator(operator: &str) -> Result<Operator, &str> {
	Ok(match operator {
		"^" | "**" => Operator::Power,
		"*" => Operator::Multiply,
		"/" => Operator::Divide,
		"+" => Operator::Add,
		"-" => Operator::Subtract,
		"==" => Operator::EqualTo,
		"!=" => Operator::NotEqualTo,
		"<" => Operator::LessThan,
		">" => Operator::GreaterThan,
		"<=" => Operator::LessThanOrEqualTo,
		">=" => Operator::GreaterThanOrEqualTo,
		"!" => Operator::Not,
		"=" => Operator::Assign,
		_ => return Err("Unknown operator")
	})
}

fn parse_token(raw_token: &str, token_type: TokenType) -> Result<Token, String> {
	if raw_token.is_empty() {
		return Err("Token is empty".to_owned());
	}

	match token_type {
			TokenType::Number => {
				let num: f64 = match raw_token.parse() {
					Ok(n) => n,
					Err(_) => return Err(format!("Failed to parse number '{}'", raw_token))
				};

				Ok(Token::Number(num))
			},
			TokenType::Variable => Ok(Token::Variable(raw_token.to_owned())),
			TokenType::Operator => {
				let operator = match parse_operator(raw_token) {
					Ok(o) => o,
					Err(err) => return Err(err.to_owned()),
				};
				Ok(Token::Operator(operator))
			},
			TokenType::Matrix => {
				let mat: Matrix = match raw_token.parse() {
					Ok(m) => m,
					Err(_) => return Err(format!("Failed to parse matrix '{}'", raw_token))
				};

				Ok(Token::Matrix(mat))
			},
			_ => Err("Token type not implemented yet.".to_owned())
		}
}

fn push_token(tokens: &mut Vec<Token>, raw_token: &str, token_type: TokenType) -> Result<(), String> {
	let token = match parse_token(raw_token , token_type) {
		Ok(t) => t,
		Err(err) => return Err(err)
	};
	tokens.push(token);
	Ok(())
}

pub fn tokenise(input: &str) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = Vec::new();

	let mut accum_type = TokenType::None;
	let mut accum = String::new();
	for char in input.chars() {
		// determine type of current token if unknown
		if accum_type == TokenType::None {
			if char.is_whitespace() {
				continue;
			} else if is_number(char) {
				accum_type = TokenType::Number;
			} else if is_matrix(char) {
				accum_type = TokenType::Matrix;
			} else if is_operator(char) {
				accum_type = TokenType::Operator;
			} else if is_variable(char) {
				accum_type = TokenType::Variable;
			}
			accum = String::new();
			accum.push(char);
			continue;
		}

		// accumulate current token
		match accum_type {
			TokenType::Number => {
				if is_number(char) {
					accum.push(char);
				} else {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			TokenType::Variable => {
				if char.is_alphabetic() {
					accum.push(char);
				} else {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			TokenType::Operator => {
				if accum.len() < 2 && is_operator(char) {
					accum.push(char);
				} else {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			TokenType::Matrix => {
				accum.push(char);

				if char == ']' {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			_ => {}
		}
	}

	if !accum.is_empty() {
		let _ = push_token(&mut tokens, &accum, accum_type);
	}

	Ok(tokens)
}
