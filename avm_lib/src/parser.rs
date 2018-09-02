use value;
use stack;

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(Option<value::Type>, &mut Vec<value::Type>) -> stack::ExecState>,
	pub line : String,
	pub line_number : i64,
}
