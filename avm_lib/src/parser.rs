use value;
use stack;

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState>,
	pub line : String,
	pub line_number : i64,
}

fn check_tokens_validity(_vec : &Vec<Token>) -> bool {
	true
}

pub fn parse_from_file(file : &String) -> Option<Vec<Token>> {
	println!("{:?}", file);
	let vec : Vec<Token> = Vec::new();
	match check_tokens_validity(&vec) {
		true => Some(vec),
		false => None,
	}
}

pub fn parse_from_stdin() -> Vec<Token> {
	println!("From stdin");
	let vec : Vec<Token> = Vec::new();
	vec
}
