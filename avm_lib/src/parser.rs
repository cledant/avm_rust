use value;

pub struct Token {
	pub val : Option<value::Type>,
	pub inst : Option<fn(value::Type, &mut Vec<value::Type>)>,
	pub line : String,
	pub line_number : i64,
}
