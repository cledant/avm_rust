use parser::Parser;
use stack::Stack;

pub struct Env {
	parser : Parser,
	stack : Stack,
}

impl Env {
	pub fn new() -> Env {
		Env { parser : Parser::new(), stack : Stack::new() }
	}

	pub fn run(self, args : &Vec<String>) {
		match args.len() {
			1  => { println!("From stdin") },
			_ => { println!("From file")},
		}
	}
}
