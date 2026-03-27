use std::collections::BTreeMap;

use crate::tokeniser::{
	Operator,
	print_token,
	Token,
	TokenResult,
	tokenise
};

use crate::colours::println_error;

pub struct Evaluator {
	variables: BTreeMap<String, Token>
}

impl Evaluator {
	pub fn new() -> Self {
		Evaluator { variables: BTreeMap::new() }
	}
	pub fn evaluate(&mut self, input: &str) {
		let tokens = match tokenise(input) {
			Ok(t) => t,
			Err(err) => {
				println_error(format!("Error: {}", err));
				return;
			}
		};

		match self.evaluate_tokens(&tokens) {
			Ok(t) => print_token(&t, &self.variables),
			Err(err) => println_error(format!("Error: {}", err))
		};
	}
	fn evaluate_tokens(&mut self, tokens: &Vec<Token>) -> TokenResult {
		let expression_list = match self.tokens_to_expressions(&tokens) {
			Ok(e) => e,
			Err(err) => {
				return Err(err.to_owned());
			}
		};

		if expression_list.len() == 0 {
			return Err("No result".to_owned());
		}
		
		match expression_list[0].evaluate(&mut self.variables) {
			Ok(t) => Ok(t),
			Err(err) => return Err(err)
		}
	}
	fn group_by_operators(&self, expressions: &mut Vec<ExpressionElement>, operators: Vec<Operator>) {
		let mut i = 1;
		while i < expressions.len() - 1 {
			let token = match &expressions[i] {
				ExpressionElement::Token(t) => t.clone(),
				ExpressionElement::Expression(_) => {
					i += 1;
					continue
				}
			};
			if let Token::Operator(operator) = &token && operators.contains(operator) {
				i -= 1;
				let slice: &Vec<ExpressionElement> = &expressions.drain(i..i+3).collect();
				let expression = ExpressionElement::Expression(Box::new(Expression {
					lhs: slice[0].clone(),
					operator: *operator,
					rhs: slice[2].clone()
				}));
				expressions.insert(i, expression);
			}

			i += 1;
		}
	}

	fn tokens_to_expressions(&mut self, tokens: &Vec<Token>) -> Result<Vec<ExpressionElement>, String> {
		let mut tokens = tokens.clone();

		// handle groupers
		{
			let mut i = 0;
			while i < tokens.len() {
				if let Token::Operator(op) = tokens[i] && op == Operator::OpenGroup {
					let start_idx = i;
					i += 1;

					let mut inner_counter = 1;
					loop {
						if i >= tokens.len() {
							return Err("Could not resolve unclosed grouped expression".to_owned());
						}
						if let Token::Operator(op) = tokens[i] {
							match op {
								Operator::OpenGroup => inner_counter += 1,
								Operator::CloseGroup => inner_counter -= 1,
								_ => {}
							}
						}

						if inner_counter == 0 {
							break;
						}

						i += 1;
					}
					if let Token::Operator(op) = tokens[i] && op != Operator::CloseGroup {
						continue;
					}

					// include grouper operators to remove from tokens
					let grouped_tokens: Vec<Token> = tokens.drain(start_idx..i + 1).collect();
					// remove grouper operators when evaluating inner expression
					let grouped_tokens: Vec<Token> = grouped_tokens[1..&grouped_tokens.len() - 1].to_vec();
					match self.evaluate_tokens(&grouped_tokens) {
						Ok(t) => {
							tokens.insert(start_idx, t);
						},
						Err(err) => return Err(err)
					};
					i = start_idx;
				}

				i += 1;
			}
		}

		let mut expressions: Vec<ExpressionElement> = tokens.iter()
			.map(|token| ExpressionElement::Token(token.clone())).collect();

		self.group_by_operators(&mut expressions, vec![Operator::Power]);
		self.group_by_operators(&mut expressions, vec![Operator::Multiply, Operator::Divide]);
		self.group_by_operators(&mut expressions, vec![Operator::Add, Operator::Subtract]);
		self.group_by_operators(&mut expressions, vec![
			Operator::EqualTo,
			Operator::NotEqualTo,
			Operator::LessThan,
			Operator::LessThanOrEqualTo,
			Operator::GreaterThan,
			Operator::GreaterThanOrEqualTo
		]);
		self.group_by_operators(&mut expressions, vec![Operator::Assign]);

		Ok(expressions)
	}
}

fn add(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Matrix(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					lhs.add_scalar(*rhs);
					Ok(Token::Matrix(lhs.clone()))
				},
				Token::Matrix(rhs) => match lhs.checked_add(rhs) {
					Ok(m) => Ok(Token::Matrix(m.clone())),
					Err(err) => Err(err.to_owned())
				},
				_ => Err("Cannot add RHS to matrix".to_owned())
			}
		},
		Token::Number(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					Ok(Token::Number(*lhs + *rhs))
				},
				Token::Matrix(rhs) => {
					rhs.add_scalar(*lhs);
					Ok(Token::Matrix(rhs.clone()))
				},
				_ => Err("Cannot add RHS to number".to_owned())
			}
		}
		_ => Err("Cannot add to LHS type".to_owned())
	}
}
fn subtract(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Matrix(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					lhs.subtract_scalar(*rhs);
					Ok(Token::Matrix(lhs.clone()))
				},
				Token::Matrix(rhs) => match &mut lhs.checked_subtract(rhs) {
					Ok(m) => Ok(Token::Matrix(m.clone())),
					Err(err) => Err(err.to_owned())
				},
				_ => Err("Cannot subtract RHS from matrix".to_owned())
			}
		},
		Token::Number(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					Ok(Token::Number(*lhs - *rhs))
				},
				_ => Err("Cannot subtract RHS from number".to_owned())
			}
		}
		_ => Err("Cannot add to LHS type".to_owned())
	}
}
fn multiply(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(*lhs * *rhs)),
			Token::Matrix(rhs) => {
				rhs.multiply_by_scalar(*lhs);
				Ok(Token::Matrix(rhs.clone()))
			},
			_ => Err("Cannot multiply number by type of RHS".to_owned())
		},
		Token::Matrix(lhs) => match rhs {
			Token::Number(rhs) => {
				lhs.multiply_by_scalar(*rhs);
				Ok(Token::Matrix(lhs.clone()))
			},
			Token::Matrix(rhs) => {
				match lhs.checked_multiply(rhs) {
					Ok(m) => Ok(Token::Matrix(m)),
					Err(err) => Err(format!("{}", err))
				}
			},
			_ => Err("Cannot multiply matrix by type of RHS".to_owned())
		}
		_ => Err("Cannot multiply type of LHS".to_owned())
	}
}
fn divide(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(*lhs / *rhs)),
			_ => Err("Cannot divide by type of RHS".to_owned())
		},
		Token::Matrix(lhs) => match rhs {
			Token::Number(rhs) => {
				lhs.divide_by_scalar(*rhs);
				Ok(Token::Matrix(lhs.clone()))
			},
			_ => Err("Can only divide matrix by a number".to_owned())
		},
		_ => Err("Cannot divide type of LHS".to_owned())
	}
}
fn power(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => {
			match rhs {
				Token::Number(rhs) => Ok(Token::Number(lhs.powf(*rhs))),
				_ => Err("Cannot raise LHS by the type of RHS".to_owned())
			}
		},
		// Token::Matrix(lhs) => {},
		_ => Err("Cannot compute power of type of LHS".to_owned())
	}
}
fn equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs == rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Cannot compare equality of different types".to_owned())
		},
		Token::Matrix(lhs) => match rhs {
			Token::Matrix(rhs) => Ok(Token::Number(if lhs.equals(rhs) { 1.0 } else { 0.0 })),
			_ => Err("Cannot compare equality of matrix to RHS type".to_owned())
		}
		_ => Err("Cannot compare equality of LHS type".to_owned())
	}
}
fn not_equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs != rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Cannot compare inequality of different types".to_owned())
		},
		Token::Matrix(lhs) => match rhs {
			Token::Matrix(rhs) => Ok(Token::Number(if !lhs.equals(rhs) { 1.0 } else { 0.0 })),
			_ => Err("Cannot compare inequality of matrix to RHS type".to_owned())
		}
		_ => Err("Cannot compare inequality of LHS type".to_owned())
	}
}
fn less_than(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs < rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with less than".to_owned())
		},
		_ => Err("Only numbers can be compared with less than".to_owned())
	}
}
fn less_than_or_equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs <= rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with less than or equal to".to_owned())
		},
		_ => Err("Only numbers can be compared with less than or equal to".to_owned())
	}
}
fn greater_than(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs > rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with greater than".to_owned())
		},
		_ => Err("Only numbers can be compared with greater than".to_owned())
	}
}
fn greater_than_or_equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs >= rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with greater than or equal to".to_owned())
		},
		_ => Err("Only numbers can be compared with greater than or equal to".to_owned())
	}
}

#[derive(Clone, Debug)]
enum ExpressionElement {
	Token(Token),
	Expression(Box<Expression>)
}

impl ExpressionElement {
	fn evaluate(&self, variables: &mut BTreeMap<String, Token>) -> TokenResult {
		let expression = match self {
			ExpressionElement::Expression(e) => e,
			ExpressionElement::Token(t) => return Ok(t.clone())
		};

		// recursively evaluate expression tree
		// if either we are still left with an expression, something has gone very bad
		let lhs = &expression.lhs;
		let mut lhs = match &expression.lhs {
			ExpressionElement::Token(t) => t.clone(),
			ExpressionElement::Expression(_) => match lhs.evaluate(variables) {
				Ok(t) => t,
				Err(err) => return Err(err)
			}
		};
		let rhs = &expression.rhs;
		let mut rhs = match &expression.rhs {
			ExpressionElement::Token(t) => t.clone(),
			ExpressionElement::Expression(_) => match rhs.evaluate(variables) {
				Ok(t) => t,
				Err(err) => return Err(err)
			}
		};

		// dereference variables (skip LHS on assign operations)
		if expression.operator != Operator::Assign && let Token::Variable(variable) = lhs {
			lhs = match variables.get(&variable) {
				Some(t) => t.clone(),
				None => return Err(format!("Could not retrieve variable '{}'", variable))
			};
		}
		if let Token::Variable(variable) = rhs {
			rhs = match variables.get(&variable) {
				Some(t) => t.clone(),
				None => return Err(format!("Could not retrieve variable '{}'", variable))
			};
		}

		match expression.operator {
			Operator::Power => power(&mut lhs, &mut rhs),
			Operator::Add => add(&mut lhs, &mut rhs),
			Operator::Subtract => subtract(&mut lhs, &mut rhs),
			Operator::Multiply => multiply(&mut lhs, &mut rhs),
			Operator::Divide => divide(&mut lhs, &mut rhs),
			Operator::EqualTo => equal_to(&mut lhs, &mut rhs),
			Operator::NotEqualTo => not_equal_to(&mut lhs, &mut rhs),
			Operator::LessThan => less_than(&mut lhs, &mut rhs),
			Operator::LessThanOrEqualTo => less_than_or_equal_to(&mut lhs, &mut rhs),
			Operator::GreaterThan => greater_than(&mut lhs, &mut rhs),
			Operator::GreaterThanOrEqualTo => greater_than_or_equal_to(&mut lhs, &mut rhs),
			Operator::Assign => {
				let variable = match lhs {
					Token::Variable(v) => v,
					_ => return Err("Can only assign to variables, maybe you meant to compare with '=='?".to_owned())
				};
				variables.insert(variable.to_string(), rhs.clone());
				Ok(rhs.clone())
			},
			#[allow(unreachable_patterns)]
			_ => return Err("Unknown operator".to_owned())
		}
	}
}

#[derive(Clone, Debug)]
struct Expression {
	lhs: ExpressionElement,
	rhs: ExpressionElement,
	operator: Operator
}
