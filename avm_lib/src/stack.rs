use parser;
use value;

pub static ERR_INVALID_TOKEN : &str = "Invalid Token";
pub static ERR_NO_EXIT_INST : &str = "No Exit Instruction";

#[derive(Debug, PartialEq)]
pub enum ExecState {
	Continue,
	Stop,
	Error(&'static str),
}

pub struct Stack {
	vec_value : Vec<value::Type>,
}

impl Stack {
	pub fn new() -> Stack {
		Stack {vec_value : Vec::new() }
	}

	pub fn run(&mut self, entry : &Vec<parser::Token>) -> ExecState {
		for token in entry.iter() {
			match token.inst {
				Some(fct) => {
					match fct(token.val.clone(),
							&mut self.vec_value) {
						ExecState::Error(e) => {
							return ExecState::Error(e);
						}
						ExecState::Stop => {
							return ExecState::Stop;
						}
						ExecState::Continue => {}
					}
				}
				None => { 
					return ExecState::Error(ERR_INVALID_TOKEN);
				}
			};
		}
		ExecState::Error(ERR_NO_EXIT_INST)
	}
}
