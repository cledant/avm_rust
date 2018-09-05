use value;
use stack;
use std::fs;

pub static ERR_OPEN_FILE : &str = "Failed to read file";

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState>,
	pub line : String,
	pub line_number : i64,
}

fn generate_tokens(content : String) -> Option<Vec<Token>> {
	let splited : Vec<_> = content.split('\n').collect();
	println!("Number of lines : {}", splited.len());
	Some(Vec::new())
}

pub fn parse_from_file(file : &String) -> Option<Vec<Token>> {
	match fs::read_to_string(file) {
		Ok(s) => generate_tokens(s),
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
