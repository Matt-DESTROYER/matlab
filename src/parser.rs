use std::ops::Range;

/*pub trait Evaluatable {
	fn evaluate(&self, lhs: &Self, rhs: &Self) -> Self;
}*/

#[derive(Clone)]
pub enum ExpressionElement<Token: PartialEq + Clone> {
	Expression(Box<Expression<Token>>),
	Token(Token)
}

#[derive(Clone)]
pub struct Expression<Token: PartialEq + Clone> {
	pub lhs: ExpressionElement<Token>,
	pub operator: Token,
	pub rhs: ExpressionElement<Token>
}

struct Parser<Token: PartialEq + Clone> {
	operators: Vec<Vec<Token>>,
	open_grouper: Token,
	close_grouper: Token
}

impl<Token: PartialEq + Clone> Parser<Token> {
	pub fn new(operators: Vec<Vec<Token>>, open_grouper: Token, close_grouper: Token) -> Self {
		Self {
			operators,
			open_grouper,
			close_grouper
		}
	}

	pub fn add_precedence_level(&mut self, operators: &Vec<Token>) {
		let operators: Vec<Token> = operators.clone();
		self.operators.push(operators);
	}

	fn find_next_group(&self, tokens: &Vec<Token>) -> Result<Option<Range<usize>>, String> {
		let mut i = 0;

		while i < tokens.len() {
			if tokens[i] == self.open_grouper {
				let start_idx = i;
				i += 1;
				let mut inner_counter = 1;
				loop {
					if i >= tokens.len() {
						return Err("Could not resolve unclosed grouped expression".to_owned());
					}

					if tokens[i] == self.open_grouper {
						inner_counter += 1;
					} else if tokens[i] == self.close_grouper {
						inner_counter -= 1;
					}

					if inner_counter == 0 {
						break;
					}

					i += 1;
				}

				if tokens[i] != self.close_grouper {
					continue;
				}

				return Ok(Some(start_idx..i + 1));
			}

			i += 1;
		}

		Ok(None)
	}

	fn generate_expressions_at_level(&self, expression_list: &mut Vec<ExpressionElement<Token>>, operators: &Vec<Token>) {
		if expression_list.len() < 2 { return }

		let mut i = 0;
		while i < expression_list.len() - 2 {
			// extract the current token
			let token = match &expression_list[i] {
				ExpressionElement::Expression(_) => continue,
				ExpressionElement::Token(t) => t
			}.clone();

			// if it's an operator in this precedence level
			if operators.contains(&token) {
				i -= 1;

				// move the surrounding elements (tokens _OR_ expressions) into a single expression
				let slice: Vec<ExpressionElement<Token>> = expression_list.drain(i..i+3).collect();
				let expression = ExpressionElement::Expression(Box::new(Expression {
					lhs: slice[0].clone(),
					operator: token,
					rhs: slice[2].clone()
				}));
				expression_list.insert(i, expression);
			}

			i += 1;
		}
	}

	pub fn parse(&self, tokens: &Vec<Token>) -> Result<ExpressionElement<Token>, String> {
		if tokens.len() == 0 { return Err("No tokens provided".to_owned()) }

		let mut expression_list: Vec<ExpressionElement<Token>> = tokens.clone().into_iter()
			.map(|token| ExpressionElement::Token(token)).collect();

		// handle groups by finding each group and parsing them first
		loop {
			let next_group = match self.find_next_group(&tokens) {
				Ok(range) => match range {
					Some(range) => range,
					None => break
				},
				Err(err) => return Err(err)
			};
			let start = next_group.start;
			
			// include grouper operators to remove from tokens
			let grouped_tokens: Vec<ExpressionElement<Token>> = expression_list.drain(next_group).collect();
			// remove grouper operators when evaluating inner expression
			let grouped_tokens: Vec<ExpressionElement<Token>> = grouped_tokens[1..&grouped_tokens.len() - 1].to_vec();
			match self.parse(&grouped_tokens) {
				Ok(expression) => {
					tokens.insert(start, expression);
				},
				Err(err) => return Err(err)
			};
		}

		// create AST by going through each precedence level and generating expressions where operators are found
		for precedence in &self.operators {
			self.generate_expressions_at_level(&mut expression_list, &precedence);
		}

		Ok(expression_list[0].clone())
	}
}
