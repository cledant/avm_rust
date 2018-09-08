use value;
use stack;
use instruction;
use std::fs;

pub static ERR_OPEN_FILE : &str = "Failed to read file";

#[derive(Debug, PartialEq)]
pub enum ParserError {
	ErrInst,
	ErrInstLexical,
	ErrValueType,
	ErrValueSyntax,
}

enum ValType {
	Char,
	Short,
	Int,
	Float,
	Double,
	None,
}

enum ValFamily {
	Integer,
	Float,
	None,
}

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState>,
	pub line : String,
	pub line_number : u64,
	pub vec_error : Vec<ParserError>,
}

#[inline]
fn are_tokens_corrects(vec_tok : &mut Vec<Token>, filename : &String) -> bool {
	//checks correct association of value and instruction	
	for tok in vec_tok.iter_mut() {
		if tok.vec_error.contains(&ParserError::ErrInstLexical)  == true {
			continue;
		}
		match (tok.inst, &tok.val) {
			(Some(inst), None) => {
				if (inst as usize == instruction::push as usize) || (inst as usize == instruction::assert as usize) {
					tok.vec_error.push(ParserError::ErrInstLexical);
				};
			}
			(Some(inst), Some(_)) => {
				if (inst as usize != instruction::push as usize) && (inst as usize != instruction::assert as usize) {
					tok.vec_error.push(ParserError::ErrInstLexical);
				};
			}
			(_, _) => {}
		};
	}
	//Display Error
	let mut is_correct = true;
	for tok in vec_tok.iter() {
		for e in tok.vec_error.iter() {
			is_correct = false;
			println!("Error In file : {} at line {}\n\t{}", filename, tok.line_number, tok.line);
			match e {
				_ => println!("=> Shit happend"),
			};
		}
	}
	is_correct
}

#[inline]
fn parse_instruction (s : &String) -> Result<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState, ParserError> {
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
		_ => Err(ParserError::ErrInst),
	}
}

#[inline]
fn parse_value_type (s : &str) -> ValType {
	match s {
		"int8" => ValType::Char,
		"int16" => ValType::Short,
		"int32" => ValType::Int,
		"float" => ValType::Float,
		"double" => ValType::Double,
		_ => ValType::None,
	}
}

#[inline]
fn parse_value_syntax (s : &str) -> ValFamily {
	let v : Vec<&str> = s.matches(".").collect();
	match v.len() {
		0 => ValFamily::Integer,
		1 => ValFamily::Float,
		_ => ValFamily::None,
	}
}

#[inline]
fn parse_value (s : &String) -> Result<value::Type, ParserError> {
	let v1 : Vec<&str> = s.matches("(").collect();
	let v2 : Vec<&str> = s.matches(")").collect();
	if let (1, 1) = (v1.len(), v2.len()) {
			let tmp : Vec<&str> = s.split(")").collect();
			let split : Vec<&str> = tmp[0].split("(").collect();
			if split.len() != 2 {
				return Err(ParserError::ErrValueSyntax)
			};
			match (parse_value_type(&split[0]), parse_value_syntax(&split[1])) {
				(ValType::Char, ValFamily::Integer) => {
					match split[1].parse::<i8>() {
						Ok(val) => return Ok(value::Type::Char(val)),
						Err(_) => return Err(ParserError::ErrValueSyntax),
					}
				},
				(ValType::Short, ValFamily::Integer) => {
					match split[1].parse::<i16>() {
						Ok(val) => return Ok(value::Type::Short(val)),
						Err(_) => return Err(ParserError::ErrValueSyntax),
					}
				},
				(ValType::Int, ValFamily::Integer) => {
					match split[1].parse::<i32>() {
						Ok(val) => return Ok(value::Type::Int(val)),
						Err(_) => return Err(ParserError::ErrValueSyntax),
					}
				},
				(ValType::Float, ValFamily::Float) => {
					match split[1].parse::<f32>() {
						Ok(val) => return Ok(value::Type::Float(val)),
						Err(_) => return Err(ParserError::ErrValueSyntax),
					}
				},
				(ValType::Double, ValFamily::Float) => {
					match split[1].parse::<f64>() {
						Ok(val) => return Ok(value::Type::Double(val)),
						Err(_) => return Err(ParserError::ErrValueSyntax),
					}
				},
				(ValType::Char, ValFamily::Float) | (ValType::Short, ValFamily::Float) | (ValType::Int, ValFamily::Float) | (ValType::Float, ValFamily::Integer) | (ValType::Double, ValFamily::Integer) => return Err(ParserError::ErrValueSyntax),
				(_, _) => return Err(ParserError::ErrValueType),
			};
	}
	Err(ParserError::ErrValueSyntax)
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
	let v_str : Vec<&str> = line.trim().split(";").collect();
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
				if vec_splited.len() > 3 {
					tok.vec_error.push(ParserError::ErrInstLexical);
				};
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
	match are_tokens_corrects(&mut vec_tok, &filename) {
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
