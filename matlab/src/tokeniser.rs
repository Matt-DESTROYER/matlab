use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::matrix::Matrix;
use crate::tools::Searchable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Power,
	Not,
	EqualTo,
	NotEqualTo,
	LessThan,
	LessThanOrEqualTo,
	GreaterThan,
	GreaterThanOrEqualTo,
	OpenGroup,
	CloseGroup,
	Assign,
	Separator
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operator::Add                  => write!(f, "Add"),
			Operator::Subtract             => write!(f, "Subtract"),
			Operator::Multiply             => write!(f, "Multiply"),
			Operator::Divide               => write!(f, "Divide"),
			Operator::Power                => write!(f, "Power"),
			Operator::Not                  => write!(f, "Not"),
			Operator::EqualTo              => write!(f, "EqualTo"),
			Operator::NotEqualTo           => write!(f, "NotEqualTo"),
			Operator::LessThan             => write!(f, "LessThan"),
			Operator::LessThanOrEqualTo    => write!(f, "LessThanOrEqualTo"),
			Operator::GreaterThan          => write!(f, "GreaterThan"),
			Operator::GreaterThanOrEqualTo => write!(f, "GreaterThanOrEqualTo"),
			Operator::OpenGroup            => write!(f, "OpenGroup"),
			Operator::CloseGroup           => write!(f, "CloseGroup"),
			Operator::Assign               => write!(f, "Assign"),
			Operator::Separator            => write!(f, "Separator")
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
	Sin,
	Cos,
	Tan
}

impl Display for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Function::Sin => write!(f, "sin"),
			Function::Cos => write!(f, "cos"),
			Function::Tan => write!(f, "tan")
		}
	}
}

#[derive(Clone, Debug)]
pub struct Tuple {
	elems: Vec<Token>
}

impl Tuple {
	pub fn new(elems: Vec<Token>) -> Self {
		Self {
			elems
		}
	}

	pub fn size(&self) -> usize {
		self.elems.len()
	}

	pub fn args(&self) -> &Vec<Token> {
		&self.elems
	}

	pub fn at(&self, index: usize) -> &Token {
		&self.elems[index]
	}
	pub fn set(&mut self, index: usize, token: &Token) {
		self.elems[index] = token.clone();
	}

	pub fn append(&mut self, token: &Token) {
		self.elems.push(token.clone());
	}
	pub fn prepend(&mut self, token: &Token) {
		self.elems.insert(0, token.clone());
	}

	pub fn append_tuple(&mut self, other: &Self) {
		self.elems.append(&mut other.elems.clone());
	}
	pub fn prepend_tuple(&mut self, other: &Self) {
		for i in 0..other.elems.len() {
			self.elems.insert(i, other.elems[i].clone());
		}
	}
}

impl Display for Tuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(")?;
		for item in &self.elems {
			write!(f, "{}", item)?;
		}
		write!(f, ")")
	}
}

#[derive(Clone, Debug)]
pub enum Token {
	Number(f64),
	Variable(String),
	Operator(Operator),
	Tuple(Tuple),
	Matrix(Matrix),
	Function(Function)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TokenType {
	None,
	Number,
	Variable,
	Operator,
	Separator,
	Function,
	Matrix
}

pub type TokenResult = Result<Token, String>;

const VALID_OPERATORS: [&str; 17] = ["^", "**", "*", "/", "+", "-", "==", "!=", "<", "<=", ">", ">=", "(", ")", "!", "=", ","];
const MAX_OPERATOR_LENGTH: usize = 2;
fn valid_operator(operator: &str) -> bool {
	VALID_OPERATORS.contains(&operator)
}

fn is_number(c: char) -> bool {
	c.is_ascii_digit() || c == '.'
}
fn is_matrix(c: char) -> bool {
	c == '['
}
fn is_operator(c: char) -> bool {
	for op in VALID_OPERATORS {
		if op.contains(c) {
			return true;
		}
	}
	false
}
fn is_variable(c: char) -> bool {
	c.is_alphabetic()
}
fn is_separator(c: char) -> bool {
	c == ','
}

fn parse_operator(operator: &str) -> Result<Operator, &str> {
	Ok(match operator {
		"^" | "**" => Operator::Power,
		"*"        => Operator::Multiply,
		"/"        => Operator::Divide,
		"+"        => Operator::Add,
		"-"        => Operator::Subtract,
		"=="       => Operator::EqualTo,
		"!="       => Operator::NotEqualTo,
		"<"        => Operator::LessThan,
		">"        => Operator::GreaterThan,
		"<="       => Operator::LessThanOrEqualTo,
		">="       => Operator::GreaterThanOrEqualTo,
		"("        => Operator::OpenGroup,
		")"        => Operator::CloseGroup,
		"!"        => Operator::Not,
		"="        => Operator::Assign,
		","        => Operator::Separator,
		_ => return Err("Operator not yet implemented")
	})
}

fn parse_function(function: &str) -> Result<Function, String> {
	Ok(match function {
		"sin" => Function::Sin,
		"cos" => Function::Cos,
		"tan" => Function::Tan,
		_ => return Err(format!("No built-in function '{}' exists", function))
	})
}

fn parse_token(raw_token: &str, token_type: TokenType) -> TokenResult {
	if raw_token.is_empty() {
		return Err("Token is empty".to_owned());
	}

	match token_type {
			TokenType::Number => match raw_token.parse() {
				Ok(n) => Ok(Token::Number(n)),
				Err(_) => Err(format!("Number parsing error: '{}'", raw_token))
			},
			TokenType::Variable => Ok(Token::Variable(raw_token.to_owned())),
			TokenType::Function => {
				let raw_token = raw_token.to_lowercase();
				match parse_function(&raw_token) {
					Ok(f) => Ok(Token::Function(f)),
					Err(err) => return Err(format!("Function parsing error: {}", err))
				}
			},
			TokenType::Operator => match parse_operator(raw_token) {
				Ok(o) => Ok(Token::Operator(o)),
				Err(err) => return Err(err.to_owned()),
			},
			TokenType::Matrix => match raw_token.parse() {
					Ok(m) => Ok(Token::Matrix(m)),
					Err(err) => return Err(format!("Matrix parsing error: '{}'", err))
			},
			_ => Err(format!("Token type not implemented yet."))
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

fn token_type(char: char) -> TokenType {
	if char.is_whitespace() {
		TokenType::None
	} else if is_separator(char) {
		TokenType::Separator
	} else if is_number(char) {
		TokenType::Number
	} else if is_matrix(char) {
		TokenType::Matrix
	} else if is_operator(char) {
		TokenType::Operator
	} else if is_variable(char) {
		TokenType::Variable
	} else {
		TokenType::None
	}
}

pub fn tokenise(input: &str) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = Vec::new();

	let mut accum_type = TokenType::None;
	let mut accum = String::new();
	for char in input.chars() {
		// determine type of current token if unknown
		if accum_type == TokenType::None {
			accum_type = token_type(char);
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
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = token_type(char);
					accum = String::from(char);
				}
			},
			TokenType::Variable => {
				if char.is_alphabetic() {
					accum.push(char);
				} else if char == '(' {
					match push_token(&mut tokens, &accum, TokenType::Function) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					match push_token(&mut tokens, &"(", TokenType::Operator) {
						Ok(_) => {},
						Err(err) => return Err(err)
					};
					accum_type = TokenType::None;
				} else {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = token_type(char);
					accum = String::from(char);
				}
			},
			TokenType::Operator => {
				if accum.len() < MAX_OPERATOR_LENGTH && is_operator(char) {
					let new_accum = accum.clone() + &char.to_string();
					if !valid_operator(&new_accum) {
						match push_token(&mut tokens, &accum, accum_type) {
							Ok(_) => {},
							Err(err) => return Err(err)
						}
						accum_type = token_type(char);
						accum = String::from(char);
						continue;
					}
					accum.push(char);
				} else {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = token_type(char);
					accum = String::from(char);
				}
			},
			TokenType::Matrix => {
				accum.push(char);

				if char == ']' {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = TokenType::None;
				}
			},
			TokenType::Separator => {
				match push_token(&mut tokens, &accum, accum_type) {
					Ok(_) => {},
					Err(err) => return Err(err)
				}
				accum_type = token_type(char);
				accum = String::from(char);
			},
			_ => return Err("Token type not yet implemented!".to_owned())
		}
	}

	if accum_type != TokenType::None && !accum.is_empty() {
		match push_token(&mut tokens, &accum, accum_type) {
			Ok(_) => {},
			Err(err) => return Err(err)
		}
	}

	Ok(tokens)
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Number(n) => write!(f, "Number({})", n),
			Token::Operator(o) => write!(f, "Operator({})", o),
			Token::Matrix(m) => write!(f, "Matrix({})", m),
			Token::Variable(v) => write!(f, "Variable({}) = ", v),
			_ => write!(f, "Display not implemented for token type")
		}
	}
}
pub fn print_token(token: &Token, variables: &BTreeMap<String, Token>) {
	match token {
		Token::Number(n) => println!("{}", n),
		Token::Operator(o) => println!("{}", o),
		Token::Matrix(m) => println!("{}", m),
		Token::Variable(v) => {
			let value = match variables.get(v) {
				Some(v) => v,
				None => return
			};
			print!("{} = ", v);
			print_token(value, variables);
			println!();
		},
		_ => {}
	}
}

impl FromStr for Matrix {
	type Err = String;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		let mut rows: Vec<Vec<f64>> = Vec::new();

		// parse matrix
		let matrix_start = match string.index_of("[") {
			Some(idx) => idx + 1,
			None => 0
		};
		let mut accum = String::new();
		let mut row: Vec<f64> = Vec::new();
		for char in string.chars().skip(matrix_start) {
			if char == ']' {
				break;
			}

			if char.is_ascii_digit() || char == '.' || char == '-' {
				accum.push(char);
			} else if char == ' ' || char == ',' || char == ';' {
				if accum.len() > 0 {
					let num: f64 = match accum.parse() {
						Ok(n) => n,
						Err(_) => return Err("Failed to parse value.".to_owned())
					};

					row.push(num);
					accum = String::new();
				}
			}
			
			if char == ';' {
				rows.push(row);
				row = Vec::new();
			}
		}

		// flush current values
		if accum.len() > 0 {
			let num: f64 = match accum.parse() {
				Ok(n) => n,
				Err(_) => return Err("Failed to parse value.".to_owned())
			};
			
			row.push(num);
		}
		if row.len() > 0 {
			rows.push(row);
		}

		// validate matrix
		if rows.len() == 0 {
			return Ok(Matrix::new(0, 0));
		}
		for row in rows.iter().skip(1) {
			if row.len() != rows[0].len() {
				return Err("Row lengths don't match.".to_owned());
			}
		}

		// populate matrix
		let mut mat = Matrix::new(rows.len(), rows[0].len());
		for row in 0..rows.len() {
			for column in 0..rows[row].len() {
				match mat.set(rows[row][column], row, column) {
					Ok(_) => {},
					Err(str) => return Err(format!("Error populating matrix: {}", str))
				}
			}
		}

		Ok(mat)
	}
}
