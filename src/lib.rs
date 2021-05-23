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
