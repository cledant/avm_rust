use parser;
use value;

pub static ERR_INVALID_TOKEN : &str = "Invalid Token";
pub static ERR_NO_EXIT_INST : &str = "No Exit Instruction";

pub enum ExecState {
	Continue,
	Stop,
	Error(&'static str),
}

pub struct Stack {
	vec_value : Vec<value::Type>,
}

impl Stack {
	pub fn run(&mut self, entry : &mut Vec<parser::Token>) {
		while let Some(token) = entry.pop() {
			match token.inst {
				Some(fct) => {
					match fct(token.val, &mut self.vec_value) {
						ExecState::Error(e) => {
							println!("Error : {:?}", e);
							return;
						}
						ExecState::Stop => { return; }
						ExecState::Continue => {}
					}
				}
				None => { 
					println!("Error : {:?}", ERR_INVALID_TOKEN);
					return;
				}
			};
		}
		println!("Error : {:?}", ERR_NO_EXIT_INST);	
	}
}
