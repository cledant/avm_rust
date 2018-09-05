use parser;
use stack::Stack;
use stack::ExecState;

pub fn run(args : &Vec<String>) {
	if args.len() == 1 {
		match Stack::new().run(&parser::parse_from_stdin()) {
			ExecState::Error(e) => println!("{}", e),
			_ => {},
		}
	} else {
		for file in args.iter().skip(1) {
			if let Some(tok) = parser::parse_from_file(&file) {
				match Stack::new().run(&tok) {
					ExecState::Error(e) => println!("File : {} : {}", file, e),
					_ => {},
				}
			}
		}
	}
}
