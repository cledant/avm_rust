use value;
use stack;
use instruction;
use std::process;
use std::fs;
use std::io;

pub static ERR_OPEN_FILE : &str = "IO Error : Failed to read file";
pub static ERR_INST : &str = "Syntax Error : Invalid Instruction";
pub static ERR_INST_LEXICAL : &str = "Lexical Error : Invalid Instruction Pattern";
pub static ERR_VALUE_TYPE : &str = "Syntax Error : Invalid Value Type";
pub static ERR_VALUE_SYNTAX : &str = "Syntax Error : Invalid Value Syntax";
pub static ERR_IO_STDIN : &str = "IO Error : Failed to read stdin. Exiting";

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
	pub vec_error : Vec<&'static str>,
}

#[inline]
fn are_tokens_corrects(vec_tok : &mut Vec<Token>, filename : &String) -> bool {
	//checks correct association of value and instruction	
	for tok in vec_tok.iter_mut() {
		if tok.vec_error.contains(&ERR_INST_LEXICAL)  == true {
			continue;
		}
		match (tok.inst, &tok.val) {
			(Some(inst), None) => {
				if ((inst as usize == instruction::push as usize) ||
					(inst as usize == instruction::assert as usize)) &&  
					(!tok.vec_error.contains(&ERR_VALUE_SYNTAX) &&
					!tok.vec_error.contains(&ERR_VALUE_TYPE)) {
						tok.vec_error.push(ERR_INST_LEXICAL);
				} else if (inst as usize != instruction::push as usize) &&
					(inst as usize != instruction::assert as usize) &&
					(tok.vec_error.contains(&ERR_VALUE_SYNTAX) ||
					tok.vec_error.contains(&ERR_VALUE_TYPE)) {
						tok.vec_error.clear();
						tok.vec_error.push(ERR_INST_LEXICAL);
				}
			}
			(Some(inst), Some(_)) => {
				if (inst as usize != instruction::push as usize) &&
					(inst as usize != instruction::assert as usize) {
						tok.vec_error.push(ERR_INST_LEXICAL);
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
			println!("=> {}", e);
		}
	}
	is_correct
}

#[inline]
fn parse_instruction (s : &String) -> Result<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState, &'static str> {
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
		_ => Err(ERR_INST),
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
	let v : Vec<&str> = s.split(".").collect();
	match v.len() {
		1 => ValFamily::Integer,
		2 => {
			match v[1].len() {
				0 => ValFamily::None,
				_ => ValFamily::Float,
			}
		},
		_ => ValFamily::None,
	}
}

#[inline]
fn parse_value (s : &String) -> Result<value::Type, &'static str> {
	let v1 : Vec<&str> = s.matches("(").collect();
	let v2 : Vec<&str> = s.matches(")").collect();
	if let (1, 1) = (v1.len(), v2.len()) {
			let tmp : Vec<&str> = s.split(")").collect();
			let split : Vec<&str> = tmp[0].split("(").collect();
			if split.len() != 2 {
				return Err(ERR_VALUE_SYNTAX)
			};
			match (parse_value_type(&split[0]), parse_value_syntax(&split[1])) {
				(ValType::Char, ValFamily::Integer) => {
					match split[1].parse::<i8>() {
						Ok(val) => return Ok(value::Type::Char(val)),
						Err(_) => return Err(ERR_VALUE_SYNTAX),
					}
				},
				(ValType::Short, ValFamily::Integer) => {
					match split[1].parse::<i16>() {
						Ok(val) => return Ok(value::Type::Short(val)),
						Err(_) => return Err(ERR_VALUE_SYNTAX),
					}
				},
				(ValType::Int, ValFamily::Integer) => {
					match split[1].parse::<i32>() {
						Ok(val) => return Ok(value::Type::Int(val)),
						Err(_) => return Err(ERR_VALUE_SYNTAX),
					}
				},
				(ValType::Float, ValFamily::Float) => {
					match split[1].parse::<f32>() {
						Ok(val) => return Ok(value::Type::Float(val)),
						Err(_) => return Err(ERR_VALUE_SYNTAX),
					}
				},
				(ValType::Double, ValFamily::Float) => {
					match split[1].parse::<f64>() {
						Ok(val) => return Ok(value::Type::Double(val)),
						Err(_) => return Err(ERR_VALUE_SYNTAX),
					}
				},
				(ValType::Char, _) | (ValType::Short, _) | (ValType::Int, _) |
				(ValType::Float, _) | (ValType::Double, _) => return Err(ERR_VALUE_SYNTAX),
				(_, _) => return Err(ERR_VALUE_TYPE),
			};
	}
	Err(ERR_VALUE_SYNTAX)
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
	//Parsing input
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
				if vec_splited.len() > 2 {
					tok.vec_error.push(ERR_INST_LEXICAL);
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
	let mut vec_tok : Vec<Token> = Vec::new();
	let mut line : u64 = 1;
	loop {
		let mut buff = String::new();
		match io::stdin().read_line(&mut buff) {
			Ok(n) => {
				//check stdin exit pattern
				buff = buff.trim().to_string();
				if (n == 0) || (buff == ";;") {
					break
				}
				//parsing input
				let mut tmp_vec_tok = Vec::new();
				tmp_vec_tok.push(generate_token(&buff.as_str(), &line));
				if let true = are_tokens_corrects(&mut tmp_vec_tok, &String::from("Stdin")) {
					vec_tok.append(&mut tmp_vec_tok);
					line = line + 1;
				}			
			}
			Err(_) => {
				println!("{}", ERR_IO_STDIN);
				process::exit(1);
			}
		}
	}
	vec_tok
}
