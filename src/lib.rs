// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

pub struct Format {
	hard_tabs: bool,
	tab_spaces: usize,
	indent_size: usize,
}

impl Default for Format {
	fn default() -> Self { Self { hard_tabs: true, tab_spaces: 4, indent_size: 0 } }
}

impl Format {
	pub fn new() -> Self { Self::default() }

	fn get_indent_string(&self) -> String {
		let mut indent_string: String = String::new();

		if self.hard_tabs {
			for _ in 0..self.indent_size {
				indent_string.push_str("\t");
			}
		} else {
			for _ in 0..self.indent_size {
				for _ in 0..self.tab_spaces {
					indent_string.push_str(" ");
				}
			}
		}

		indent_string
	}

	fn literal(&mut self, literal: Literal) -> String {
		match literal {
			Literal::Null => format!("null"),
			Literal::Integer(integer) => format!("{}", integer),
			Literal::Float(float) => format!("{}", big_rational_to_string(float)),
			Literal::Boolean(boolean) => format!("{}", boolean),
			Literal::String(string) => format!("\"{}\"", string),
			Literal::Vec(vector) => format!("[{}]", {
				let mut fmt_string: String = String::new();
				for (i, expression) in vector.iter().enumerate() {
					fmt_string.push_str(&format!("{}", self.expression(expression.clone())));
					if i < vector.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				fmt_string
			}),
			Literal::Tuple(tuple) => format!("({})", {
				let mut fmt_string: String = String::new();
				for (i, expression) in tuple.iter().enumerate() {
					fmt_string.push_str(&format!("{}", self.expression(expression.clone())));
					if i < tuple.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				fmt_string
			}),
			Literal::HashMap(hashmap) => format!("{{{}}}", {
				let mut fmt_string: String = String::new();
				for (i, (key, value)) in hashmap.iter().enumerate() {
					fmt_string.push_str(&format!(
						"{}: {}",
						self.expression(key.clone()),
						self.expression(value.clone())
					));
					if i < hashmap.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				fmt_string
			}),
		}
	}

	fn expression(&mut self, expression: Expression) -> String {
		use Expression::*;

		match expression {
			Expression::Identifier(identifier) => identifier,
			Expression::Literal(literal) => self.literal(literal),
			Expression::Prefix(prefix, lhs) => format!("{}{}", prefix, self.expression(*lhs)),
			Expression::Infix(infix, left, right) => {
				format!("{} {} {}", self.expression(*left), infix, self.expression(*right))
			},
			Expression::Fn { params, body } => format!(
				"fn ({}) {}",
				{
					let mut fmt_string: String = String::new();
					for (i, param) in params.iter().enumerate() {
						fmt_string.push_str(&format!("{}", param));
						if i < params.len() - 1 {
							fmt_string.push_str(", ");
						}
					}
					fmt_string
				},
				self.block(body)
			),
			Call { function, arguments } => format!("{}({})", self.expression(*function), {
				let mut fmt_string: String = String::new();
				for (i, expression) in arguments.iter().enumerate() {
					fmt_string.push_str(&format!("{}", self.expression(expression.clone())));
					if i < arguments.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				fmt_string
			}),
			Index(left, index) => {
				format!("{}[{}]", self.expression(*left), self.expression(*index))
			},
		}
	}

	fn block(&mut self, block: Block) -> String {
		self.indent_size += 1;
		let mut result: String = String::new();

		if block.0.len() == 0 {
			self.indent_size -= 1;
			return format!("{{}}");
		}

		for statement in block.0 {
			result.push_str(&self.get_indent_string());
			result.push_str(&self.statement(statement));
		}

		self.indent_size -= 1;
		format!("{{\n{}{}}}", result, self.get_indent_string())
	}

	fn statement(&mut self, statement: Statement) -> String {
		let mut result = match statement {
			Statement::Let(name, value) => format!("let {} = {};", name, self.expression(value)),
			Statement::Expression(expression) => format!("{};", self.expression(expression)),
			Statement::ExpressionReturn(expression) => self.expression(expression),
			Statement::Fn { name, params, body } => format!(
				"fn {}({}) {}",
				name,
				{
					let mut fmt_string: String = String::new();
					for (i, param) in params.iter().enumerate() {
						fmt_string.push_str(&format!("{}", param));
						if i < params.len() - 1 {
							fmt_string.push_str(", ");
						}
					}
					fmt_string
				},
				self.block(body)
			),
			Statement::Import(name) => format!("import \"{}\";", name),
		};

		result.push_str("\n");
		result
	}

	pub fn run_with_parser(&mut self, mut parser: Parser) -> Result<String, Exception> {
		let mut result: String = String::new();

		loop {
			result.push_str(&self.statement(match parser.next()? {
				Some(statement) => statement,
				None => break,
			}));
		}

		Ok(result)
	}

	pub fn run(&mut self, ast: AbstractSyntaxTree) -> String { self.block(Block(ast.statements)) }
}
