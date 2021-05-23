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
