use crate::Float;
use anyhow::Result;
use std::{fs::File, io::Read};

const NEW_LINE: u8 = 10;
const QUOTE: u8 = 34;
const HASH: u8 = 35;
const NEGATIVE: u8 = 45;
const POINT: u8 = 46;
const OPEN_BRACKET: u8 = 91;
const CLOSE_BRACKET: u8 = 93;

static FILENAME: &str = "";

pub fn parse_files(target: &mut impl ParserTarget, filenames: &[String]) {
	todo!()
}

pub fn parse_string(target: &mut impl ParserTarget, s: &str) {
	todo!()
}

#[derive(Debug, Clone, Copy)]
pub struct FileLOC<'a> {
	pub filename: &'a str,
	pub line: u32,
	pub column: u32,
}

impl<'a> FileLOC<'a> {
	pub const fn new(filename: &'a str) -> Self {
		Self { filename, line: 0, column: 0 }
	}
}

#[derive(Debug)]
pub struct Token<'a> {
	pub token: &'a str,
	pub token_type: TokenType,
	pub loc: FileLOC<'a>,
}

impl<'a> std::fmt::Display for Token<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.token.fmt(f)
	}
}

/// Only checks `token` and `token_type` for equality.
impl<'a> PartialEq for Token<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.token == other.token && self.token_type == other.token_type
	}
}

impl<'a> Eq for Token<'a> {}

/// Only checks `token` and `token_type` for equality.
impl<'a> PartialEq<Token<'_>> for &Token<'_> {
	fn eq(&self, other: &Token<'_>) -> bool {
		(*self).eq(other)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
	Directive,
	Number,
	Comment,
	String,
	OpenBracket,
	CloseBracket,
	Error(String),
}

impl TokenType {
	pub fn error(msg: impl ToString) -> Self {
		TokenType::Error(msg.to_string())
	}
}

pub struct Tokenizer<'a> {
	content: String,
	filename: &'a str,
}

impl<'a> Tokenizer<'a> {
	pub fn new(content: String, filename: &'a str) -> Self {
		Self { content, filename }
	}

	pub fn from_file(filename: &'a str) -> Result<Self> {
		let mut contents = String::new();
		File::open(filename)?.read_to_string(&mut contents)?;

		Ok(Self::new(contents, filename))
	}

	pub fn from_string(contents: String) -> Self {
		Self::new(contents, FILENAME)
	}

	pub fn from_str(contents: &str) -> Self {
		Self::from_string(contents.to_string())
	}

	pub fn iter(&self) -> TokenizerIter<'_> {
		let loc = FileLOC::new(self.filename);

		TokenizerIter { tokenizer: self, p: 0, loc }
	}
}

impl<'a> IntoIterator for &'a Tokenizer<'a> {
	type Item = Token<'a>;
	type IntoIter = TokenizerIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

pub struct TokenizerIter<'a> {
	tokenizer: &'a Tokenizer<'a>,
	// current position
	p: usize,
	loc: FileLOC<'a>,
}

impl<'a> TokenizerIter<'a> {
	fn get_u8(&mut self) -> Option<u8> {
		if self.is_eof() {
			return None;
		}

		let ret = self.current_u8();
		self.p += 1;
		if ret == NEW_LINE {
			self.loc.line += 1;
			self.loc.column = 0;
		} else {
			self.loc.column += 1;
		}

		Some(ret)
	}

	fn current_u8(&self) -> u8 {
		self.tokenizer.content.as_bytes()[self.p]
	}

	fn is_eof(&self) -> bool {
		self.p >= self.tokenizer.content.len()
	}

	fn current_is_number_start(&self) -> bool {
		self.current_u8() == NEGATIVE || self.current_is_number()
	}

	fn current_is_number(&self) -> bool {
		self.current_u8() == POINT || self.current_u8().is_ascii_digit()
	}

	fn current_is_e(&self) -> bool {
		self.current_u8() == 101 || self.current_u8() == 79
	}

	fn handle_string(&mut self) -> TokenType {
		self.get_u8();
		while !self.is_eof() && self.current_u8() != QUOTE && self.current_u8() != NEW_LINE {
			self.get_u8();
		}
		if self.is_eof() {
			return TokenType::error("premature EOF");
		} else if self.current_u8() == NEW_LINE {
			self.get_u8();
			return TokenType::error("unterminated string");
		}
		self.get_u8();
		TokenType::String
	}

	fn handle_directive_and_number(&mut self) -> TokenType {
		if self.current_is_number_start() {
			self.get_number();
			if !self.is_eof() && self.current_is_e() {
				self.get_u8();
				self.get_number();
			}

			TokenType::Number
		} else if self.current_u8().is_ascii_alphabetic() {
			self.get_u8();
			while !self.is_eof() && self.current_u8().is_ascii_alphabetic() {
				self.get_u8();
			}

			TokenType::Directive
		} else {
			self.get_u8();
			TokenType::Error("unknown token".to_string())
		}
	}

	fn get_number(&mut self) {
		assert!(self.current_is_number_start());
		self.get_u8();
		while !self.is_eof() && self.current_is_number() {
			self.get_u8();
		}
	}
}

impl<'a> Iterator for TokenizerIter<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while !self.is_eof() && !self.current_u8().is_ascii_graphic() {
			self.get_u8();
		}

		if self.is_eof() {
			return None;
		}

		let start = self.p;
		let loc = self.loc;
		let token_type = match self.current_u8() {
			OPEN_BRACKET => {
				self.get_u8();
				TokenType::OpenBracket
			}
			CLOSE_BRACKET => {
				self.get_u8();
				TokenType::CloseBracket
			}
			QUOTE => self.handle_string(),
			HASH => {
				self.get_u8();
				while !self.is_eof() && self.current_u8() != NEW_LINE {
					self.get_u8();
				}
				TokenType::Comment
			}
			_ => self.handle_directive_and_number(),
		};

		Some(Token { token: &self.tokenizer.content[start..self.p], token_type, loc })
	}
}

#[derive(Debug)]
pub struct ParsedParameter<'a> {
	pub r#type: String,
	pub name: String,
	pub loc: FileLOC<'a>,
	pub floats: Vec<Float>,
	pub ints: Vec<i32>,
	pub strings: Vec<String>,
	pub bools: Vec<bool>,
	pub looked_up: bool,
}

impl<'a> ParsedParameter<'a> {
	pub fn new(loc: FileLOC<'a>) -> Self {
		Self {
			r#type: Default::default(),
			name: Default::default(),
			loc,
			floats: Default::default(),
			ints: Default::default(),
			strings: Default::default(),
			bools: Default::default(),
			looked_up: Default::default(),
		}
	}

	pub fn add_float(&mut self, v: Float) {
		self.floats.push(v);
	}

	pub fn add_int(&mut self, i: i32) {
		self.ints.push(i);
	}

	pub fn add_string(&mut self, s: &str) {
		self.strings.push(s.to_string());
	}

	pub fn add_bool(&mut self, v: bool) {
		self.bools.push(v);
	}
}

pub type ParsedParameterVector<'a> = Vec<ParsedParameter<'a>>;

pub trait ParserTarget {
	fn scale(&mut self, sx: Float, sy: Float, sz: Float, loc: FileLOC);
	fn shape(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn option(&mut self, name: &str, value: &str, loc: FileLOC);
	fn identity(&mut self, loc: FileLOC);
	fn translate(&mut self, dx: Float, dy: Float, dz: Float, loc: FileLOC);
	fn rotate(&mut self, angle: Float, ax: Float, ay: Float, az: Float, loc: FileLOC);
	fn look_at(
		&mut self,
		ex: Float,
		ey: Float,
		ez: Float,
		lx: Float,
		ly: Float,
		lz: Float,
		ux: Float,
		uy: Float,
		uz: Float,
		loc: FileLOC,
	);
	fn concat_transform(&mut self, transform: [Float; 16], loc: FileLOC);
	fn transform(&mut self, transform: [Float; 16], loc: FileLOC);
	fn coordinate_system(&mut self, name: &str, loc: FileLOC);
	fn coord_sys_transform(&mut self, name: &str, loc: FileLOC);
	fn active_transform_all(&mut self, loc: FileLOC);
	fn active_transform_end_time(&mut self, loc: FileLOC);
	fn active_transform_start_time(&mut self, loc: FileLOC);
	fn transform_times(&mut self, start: Float, end: Float, loc: FileLOC);
	fn color_space(&mut self, space: &str, loc: FileLOC);
	fn pixel_filter(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn film(&mut self, r#type: &str, params: ParsedParameterVector, loc: FileLOC);
	fn accelerator(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn integrator(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn camera(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn make_named_medium(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn medium_interface(&mut self, inside_name: &str, outside_name: &str, loc: FileLOC);
	fn sampler(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn world_begin(&mut self, loc: FileLOC);
	fn attribute_begin(&mut self, loc: FileLOC);
	fn attribute_end(&mut self, loc: FileLOC);
	fn attribute(&mut self, target: &str, params: ParsedParameterVector, loc: FileLOC);
	fn texture(
		&mut self,
		name: &str,
		r#type: &str,
		texname: &str,
		params: ParsedParameterVector,
		loc: FileLOC,
	);
	fn material(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn make_named_material(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn named_material(&mut self, name: &str, loc: FileLOC);
	fn light_source(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn area_light_source(&mut self, name: &str, params: ParsedParameterVector, loc: FileLOC);
	fn reverse_orientation(&mut self, loc: FileLOC);
	fn object_begin(&mut self, name: &str, loc: FileLOC);
	fn object_end(&mut self, loc: FileLOC);
	fn object_instance(&mut self, name: &str, loc: FileLOC);
	fn end_of_files(&mut self);
}

#[cfg(test)]
mod tests {
	use super::*;

	fn new_token(token: &str, token_type: TokenType) -> Token<'_> {
		static LOC: FileLOC = FileLOC::new(FILENAME);
		Token { token, token_type, loc: LOC }
	}

	fn check_tokens(t: Tokenizer, v: &[Token]) {
		for (i, a) in t.iter().enumerate() {
			assert_eq!(a, v[i]);
		}
	}

	#[test]
	fn test_tokenizer_basic() {
		let t = Tokenizer::from_str("Shape \"sphere\" \"float radius\" [1]");
		let v = [
			new_token("Shape", TokenType::Directive),
			new_token("\"sphere\"", TokenType::String),
			new_token("\"float radius\"", TokenType::String),
			new_token("[", TokenType::OpenBracket),
			new_token("1", TokenType::Number),
			new_token("]", TokenType::CloseBracket),
		];
		check_tokens(t, &v);

		let t = Tokenizer::from_str("Shape \"sphere\"\n\"float radius\" [1]");
		let v = [
			new_token("Shape", TokenType::Directive),
			new_token("\"sphere\"", TokenType::String),
			new_token("\"float radius\"", TokenType::String),
			new_token("[", TokenType::OpenBracket),
			new_token("1", TokenType::Number),
			new_token("]", TokenType::CloseBracket),
		];
		check_tokens(t, &v);

		// TBD: does PBRT support escape characters?
		let t = Tokenizer::from_str(
			r#"Shape"sphere" # foo bar [
"float radius" 1"#,
		);
		let v = [
			new_token("Shape", TokenType::Directive),
			new_token("\"sphere\"", TokenType::String),
			new_token("# foo bar [", TokenType::Comment),
			new_token("\"float radius\"", TokenType::String),
			new_token("1", TokenType::Number),
		];
		check_tokens(t, &v);

		let t = Tokenizer::from_str(
			r#"WorldBegin # hello
Integrator "deep" "float density" [ 2 2.66612 -5e-51]
"#,
		);
		let v = [
			new_token("WorldBegin", TokenType::Directive),
			new_token("# hello", TokenType::Comment),
			new_token("Integrator", TokenType::Directive),
			new_token("\"deep\"", TokenType::String),
			new_token("\"float density\"", TokenType::String),
			new_token("[", TokenType::OpenBracket),
			new_token("2", TokenType::Number),
			new_token("2.66612", TokenType::Number),
			new_token("-5e-51", TokenType::Number),
			new_token("]", TokenType::CloseBracket),
		];
		check_tokens(t, &v);
	}

	#[test]
	fn test_tokenizer_errors() {
		let t = Tokenizer::from_str("Shape\"sphere\"\t\t # foo bar\n\"float radius");
		let v = [
			new_token("Shape", TokenType::Directive),
			new_token("\"sphere\"", TokenType::String),
			new_token("# foo bar", TokenType::Comment),
			new_token("\"float radius", TokenType::error("premature EOF")),
		];
		check_tokens(t, &v);

		let t = Tokenizer::from_str("Shape\"sphere\"\t\t # foo bar\n\"float radius\\");
		let v = [
			new_token("Shape", TokenType::Directive),
			new_token("\"sphere\"", TokenType::String),
			new_token("# foo bar", TokenType::Comment),
			new_token("\"float radius\\", TokenType::error("premature EOF")),
		];
		check_tokens(t, &v);

		let t = Tokenizer::from_str("Shape\"sphere\"\t\t # foo bar\n\"float radius\n\" 5");
		let v = [
			new_token("Shape", TokenType::Directive),
			new_token("\"sphere\"", TokenType::String),
			new_token("# foo bar", TokenType::Comment),
			new_token("\"float radius\n", TokenType::error("unterminated string")),
			new_token("\" 5", TokenType::error("premature EOF")),
		];
		check_tokens(t, &v);
	}

	#[test]
	fn test_tokenizer_example() {
		let t = Tokenizer::from_file("tests/assets/example.pbrt").unwrap();
		let a: Vec<_> = t.iter().collect();
		assert_eq!(a[0], new_token("LookAt", TokenType::Directive));
		assert_eq!(a.last().unwrap(), new_token("AttributeEnd", TokenType::Directive));
	}
}
