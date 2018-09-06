use value;
use stack;
use instruction;
use std::fs;

pub static ERR_OPEN_FILE : &str = "Failed to read file";

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState>,
	pub line : String,
	pub line_number : u64,
}

#[inline]
fn are_tokens_corrects(_vec_tok : &Vec<Token>, _filename : &String) -> bool {
	true
}

#[inline]
fn generate_token(line : &str, line_nb : &u64) -> Token {
	let string_line = String::from(line);
	let mut splited_line = string_line.split_whitespace();
	let mut tok = Token {
		val : None,
		inst : None,
		line : String::from(line),
		line_number : *line_nb,
	};
	while let Some(_word) =  splited_line.next() {
		match (&tok.inst, &tok.val) {
			(_, _) => {
				tok.inst = Some(instruction::exit);
			},
		}
	};
	tok
}

#[inline]
fn generate_vec_token(content : &String, filename : &String) -> Option<Vec<Token>> {
	let mut splited_file = content.lines();
	let mut vec_tok : Vec<Token> = Vec::new();
	let mut line_nb : u64 = 1;
	while let Some(line) = splited_file.next() {
		match (line.len(), generate_token(&line, &line_nb)) {
			(0, _) => {},
			(_, tok) => vec_tok.push(tok),
		}
		line_nb = line_nb + 1;
	}
	match are_tokens_corrects(&vec_tok, &filename) {
		true => Some(vec_tok),
		false => None,
	}
}

pub fn parse_from_file(file : &String) -> Option<Vec<Token>> {
	match fs::read_to_string(file) {
		Ok(s) => generate_vec_token(&s, &file),
		Err(_) => {
			println!("File : {} : {}", file, ERR_OPEN_FILE);
			None
		},
	}
}

pub fn parse_from_stdin() -> Vec<Token> {
	println!("From stdin");
	let vec : Vec<Token> = Vec::new();
	vec
}
