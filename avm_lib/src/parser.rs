use value;
use stack;
use instruction;
use std::fs;

pub static ERR_OPEN_FILE : &str = "Failed to read file";

#[derive(Debug, PartialEq)]
pub enum ErrorType {
	Err_Inst,
	Err_Inst_Lexical,
	Err_Value_Type,
	Err_Value_Syntax,
}

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState>,
	pub line : String,
	pub line_number : u64,
	pub vec_error : Vec<ErrorType>,
}

#[inline]
fn are_tokens_corrects(_vec_tok : &Vec<Token>, _filename : &String) -> bool {
	true
}

#[inline]
fn parse_instruction (s : &String) -> Result<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState, ErrorType> {
	match s.as_ref() {
		"push" => Ok(instruction::push),
		"pop" => Ok(instruction::pop),
		"dump" => Ok(instruction::dump),
		"assert" => Ok(instruction::assert),
		"add" => Ok(instruction::add),
		"sub" => Ok(instruction::sub),
		"mul" => Ok(instruction::mul),
		"div" => Ok(instruction::div),
		"mod" => Ok(instruction::rem),
		"print" => Ok(instruction::print),
		"exit" => Ok(instruction::exit),
		_ => Err(ErrorType::Err_Inst),
	}
}

fn parse_value (s : &String) -> Result<value::Type, ErrorType> {
	Ok(value::Type::Int(42))
}

#[inline]
fn generate_token(line : &str, line_nb : &u64) -> Token {
	let mut tok = Token {
		val : None,
		inst : None,
		line : String::from(line),
		line_number : *line_nb,
		vec_error : Vec::new(),
	};

	//setup line to be parsed
	let v_str : Vec<&str> = line.trim().split(";;").collect();
	let mut iter = v_str[0].split_whitespace();
	let mut vec_splited : Vec<String> = Vec::new(); 
	while let Some(word) =  iter.next() {
		vec_splited.push(String::from(word));
	};

	 match vec_splited.len() {
		0 => { return tok; },
		1 => {
				match parse_instruction(&vec_splited[0]) {
					Ok(inst) => tok.inst = Some(inst),
					Err(e) => tok.vec_error.push(e),
				}
		}
		_ => {
				match parse_instruction(&vec_splited[0]) {
					Ok(inst) => tok.inst = Some(inst),
					Err(e) => tok.vec_error.push(e),
				}
				match parse_value(&vec_splited[1]) {
					Ok(val) => tok.val = Some(val),
					Err(e) => tok.vec_error.push(e),
				}
		}
	}
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
