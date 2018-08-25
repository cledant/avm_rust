pub mod operand {

extern crate num;

use std::process;
use std::ops::Add;

	pub enum Value {
		Char(i8),
		Short(i16),
		Int(i32),
		Float(f32),
		Double(f64),
	}	

	fn checked_add_integer<T : num::CheckedAdd> (x : T, y : T) -> T {
		let val = x.checked_add(&y);
		match val {
			Some(x) => x,
			None => {
				println!("Overflow detected ! Exiting !");
				process::exit(1)
			}
		}
	}	

	fn checked_add_float<T : num::Float> (x : T, y : T) -> T {
		let val = x + y;
		if val.is_finite() {
			val
		} else {
			println!("Overflow detected ! Exiting !");
			process::exit(1)
		}
	}

	impl Add for Value {
		type Output = Value;

		fn add(self, other : Value) -> Value {
			match (self, other) {
				//Char
				(Value::Char(x), Value::Char(y)) => {
					Value::Char(checked_add_integer(x, y))
				}
				(Value::Short(x), Value::Char(y)) => {
					Value::Short(checked_add_integer(x, y as i16))
				}
				(Value::Int(x), Value::Char(y)) => {
					Value::Int(checked_add_integer(x, y as i32))
				}
				(Value::Float(x), Value::Char(y)) => {
					Value::Float(checked_add_float(x, y as f32))
				}
				(Value::Double(x), Value::Char(y)) => {
					Value::Double(checked_add_float(x, y as f64))
				}
				//Short
				(Value::Char(x), Value::Short(y)) => {
					Value::Short(checked_add_integer(x as i16, y))
				}
				(Value::Short(x), Value::Short(y)) => {
					Value::Short(checked_add_integer(x, y))
				}
				(Value::Int(x), Value::Short(y)) => {
					Value::Int(checked_add_integer(x, y as i32))
				}
				(Value::Float(x), Value::Short(y)) => {
					Value::Float(checked_add_float(x, y as f32))
				}
				(Value::Double(x), Value::Short(y)) => {
					Value::Double(checked_add_float(x, y as f64))
				}
				//Int
				(Value::Char(x), Value::Int(y)) => {
					Value::Int(checked_add_integer(x as i32, y))
				}
				(Value::Short(x), Value::Int(y)) => {
					Value::Int(checked_add_integer(x as i32, y))
				}
				(Value::Int(x), Value::Int(y)) => {
					Value::Int(checked_add_integer(x, y))
				}
				(Value::Float(x), Value::Int(y)) => {
					Value::Float(checked_add_float(x, y as f32))
				}
				(Value::Double(x), Value::Int(y)) => {
					Value::Double(checked_add_float(x, y as f64))
				}
				//Float
				(Value::Char(x), Value::Float(y)) => {
					Value::Float(checked_add_float(x as f32, y))
				}
				(Value::Short(x), Value::Float(y)) => {
					Value::Float(checked_add_float(x as f32, y))
				}
				(Value::Int(x), Value::Float(y)) => {
					Value::Float(checked_add_float(x as f32, y))
				}
				(Value::Float(x), Value::Float(y)) => {
					Value::Float(checked_add_float(x, y))
				}
				(Value::Double(x), Value::Float(y)) => {
					Value::Double(checked_add_float(x, y as f64))
				}
				//Double
				(Value::Char(x), Value::Double(y)) => {
					Value::Double(checked_add_float(x as f64, y))
				}
				(Value::Short(x), Value::Double(y)) => {
					Value::Double(checked_add_float(x as f64, y))
				}
				(Value::Int(x), Value::Double(y)) => {
					Value::Double(checked_add_float(x as f64, y))
				}
				(Value::Float(x), Value::Double(y)) => {
					Value::Double(checked_add_float(x as f64, y))
				}
				(Value::Double(x), Value::Double(y)) => {
					Value::Double(checked_add_float(x, y))
				}
			}
		}
	}
}
