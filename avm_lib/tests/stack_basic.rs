extern crate avm_lib;

use avm_lib::stack;
use avm_lib::parser::Token;
use avm_lib::value::Type;
use avm_lib::instruction;
use avm_lib::stack::ExecState;

#[test]
fn stack_no_exit() {
	//Init
	let mut stack : stack::Stack = stack::Stack::new();
	let mut vec_tok : Vec<Token> = Vec::new();
	//Test
	assert_eq!(ExecState::Error(stack::ERR_NO_EXIT_INST),
		stack.run(&mut vec_tok));
}

#[test]
fn stack_push() {
	//Init
	let mut stack : stack::Stack = stack::Stack::new();
	let mut vec_tok : Vec<Token> = Vec::new();
	let tok = Token {val : Some(Type::Char(42)),
			inst : Some(instruction::push),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : None,
			inst : Some(instruction::exit),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	//Test
	assert_eq!(ExecState::Stop, stack.run(&mut vec_tok));
}

#[test]
fn stack_dump() {
	//Init
	let mut stack : stack::Stack = stack::Stack::new();
	let mut vec_tok : Vec<Token> = Vec::new();
	let tok = Token {val : Some(Type::Char(10)),
			inst : Some(instruction::push),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : Some(Type::Short(20)),
			inst : Some(instruction::push),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : Some(Type::Int(30)),
			inst : Some(instruction::push),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : Some(Type::Float(21.21)),
			inst : Some(instruction::push),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : Some(Type::Double(42.42)),
			inst : Some(instruction::push),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : None,
			inst : Some(instruction::dump),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	let tok = Token {val : None,
			inst : Some(instruction::exit),
			line : String::from("empty") , line_number : 0};
	vec_tok.push(tok);
	//Test
	assert_eq!(ExecState::Stop, stack.run(&mut vec_tok));
}
