use value;
use stack;
use stack::ExecState;

pub static ERR_EMPTY_STACK : &str = "Stack is empty";
pub static ERR_FAILED_ASSERT : &str = "Stack top value if different from assert";
pub static ERR_IMPOSSIBLE : &str = "Impossible case happend";
pub static ERR_NOT_CHAR_TYPE : &str = "Value to print is not an Int8";

#[inline]
pub fn push (val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match val {
		Some(x) => {
			vec.push(x);
			ExecState::Continue	
		}
		None => ExecState::Error(ERR_IMPOSSIBLE)
	}
}

#[inline]
pub fn pop (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match vec.pop() {
		Some(_) => ExecState::Continue,
		None => ExecState::Error(ERR_EMPTY_STACK),
	}	
}

#[inline]
pub fn dump (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	for value in vec.iter().rev() {
		value.print();
	}
	ExecState::Continue
}

#[inline]
pub fn assert (val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match (val , vec.last()) {
		(Some(x), Some(top)) => {
			if x == *top {
				ExecState::Continue
			} else {
				ExecState::Error(ERR_FAILED_ASSERT)
			}
		},
		(Some(_), None) => ExecState::Error(ERR_EMPTY_STACK),
		(None, _) => ExecState::Error(ERR_IMPOSSIBLE),
	}
}

#[inline]
pub fn add (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match (vec.pop(), vec.pop()) {
		(Some(x), Some(y)) => {
			match  y + x {
				Ok(val) => { 
					vec.push(val);
					ExecState::Continue
				}
				Err(e) => ExecState::Error(e),
			}
		}
		(_, None) => ExecState::Error(ERR_EMPTY_STACK),
		(None, _) => ExecState::Error(ERR_EMPTY_STACK),
	}
}

#[inline]
pub fn sub (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match (vec.pop(), vec.pop()) {
		(Some(x), Some(y)) => {
			match  y - x {
				Ok(val) => { 
					vec.push(val);
					ExecState::Continue
				}
				Err(e) => ExecState::Error(e),
			}
		}
		(_, None) => ExecState::Error(ERR_EMPTY_STACK),
		(None, _) => ExecState::Error(ERR_EMPTY_STACK),
	}
}

#[inline]
pub fn mul (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match (vec.pop(), vec.pop()) {
		(Some(x), Some(y)) => {
			match  y * x {
				Ok(val) => { 
					vec.push(val);
					ExecState::Continue
				}
				Err(e) => ExecState::Error(e),
			}
		}
		(_, None) => ExecState::Error(ERR_EMPTY_STACK),
		(None, _) => ExecState::Error(ERR_EMPTY_STACK),
	}
}

#[inline]
pub fn div (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match (vec.pop(), vec.pop()) {
		(Some(x), Some(y)) => {
			match  y / x {
				Ok(val) => { 
					vec.push(val);
					ExecState::Continue
				}
				Err(e) => ExecState::Error(e),
			}
		}
		(_, None) => ExecState::Error(ERR_EMPTY_STACK),
		(None, _) => ExecState::Error(ERR_EMPTY_STACK),
	}
}

#[inline]
pub fn rem (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match (vec.pop(), vec.pop()) {
		(Some(x), Some(y)) => {
			match  y % x {
				Ok(val) => { 
					vec.push(val);
					ExecState::Continue
				}
				Err(e) => ExecState::Error(e),
			}
		}
		(_, None) => ExecState::Error(ERR_EMPTY_STACK),
		(None, _) => ExecState::Error(ERR_EMPTY_STACK),
	}
}

#[inline]
pub fn print (_val : Option<value::Type>, vec : &mut Vec<value::Type>) -> stack::ExecState {
	match vec.last() {
		Some(value) => {
			if let value::Type::Char(c) = value {
				let cvrt = *c as u8 as char;
				if !cvrt.is_ascii_control() {
					print!("{:?}", cvrt);
				}
				ExecState::Continue
			} else {
				ExecState::Error(ERR_NOT_CHAR_TYPE)
			}
		}
		None => ExecState::Error(ERR_EMPTY_STACK),
	}
}

#[inline]
pub fn exit (_val : Option<value::Type>, _vec : &mut Vec<value::Type>) -> stack::ExecState {
	ExecState::Stop
}
