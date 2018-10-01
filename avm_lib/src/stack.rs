use parser;
use value::Type;

pub static ERR_INVALID_TOKEN: &str = "Runtime error : Invalid Token";
pub static ERR_NO_EXIT_INST: &str = "Runtime error : No Exit Instruction";

#[derive(Debug, PartialEq)]
pub enum ExecState {
    Continue,
    Stop,
    Error(&'static str),
}

pub fn run(entry: &Vec<parser::Token>) -> ExecState {
    let mut vec_value: Vec<Type> = Vec::new();
    for token in entry.iter() {
        if let Some(fct) = token.inst {
            match fct(token.val.clone(), &mut vec_value) {
                ExecState::Error(e) => return ExecState::Error(e),
                ExecState::Stop => return ExecState::Stop,
                ExecState::Continue => {}
            }
        };
    }
    ExecState::Error(ERR_NO_EXIT_INST)
}
