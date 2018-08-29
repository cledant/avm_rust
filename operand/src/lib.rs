pub mod operand {

extern crate num;

use std::ops::Add;

	pub enum Value {
		Char(i8),
		Short(i16),
		Int(i32),
		Float(f32),
		Double(f64),
	}	

	static ERR_OVERFLOW : &str = "Overflow detected";

	#[inline]
	fn checked_add_float<T : num::Float> (x : T, y : T) -> Result<T, &'static str> {
		let val = x + y;
		if val.is_finite() {
			Ok(val)
		} else {
			Err(ERR_OVERFLOW)
		}
	}

	impl Add for Value {
		type Output = Result<Value, &'static str>;

		fn add(self, other : Value) -> Result<Value, &'static str> {
			match (self, other) {
				//Char
				(Value::Char(x), Value::Char(y)) => {
					match x.checked_add(y) {
						Some(nb) => Ok(Value::Char(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Short(x), Value::Char(y)) => {
					match x.checked_add(y as i16) {
						Some(nb) => Ok(Value::Short(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Int(x), Value::Char(y)) => {
					match x.checked_add(y as i32) {
						Some(nb) => Ok(Value::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Float(x), Value::Char(y)) => {
					match checked_add_float(x, y as f32) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Double(x), Value::Char(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Short
				(Value::Char(x), Value::Short(y)) => {
					match y.checked_add(x as i16) {
						Some(nb) => Ok(Value::Short(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Short(x), Value::Short(y)) => {
					match x.checked_add(y) {
						Some(nb) => Ok(Value::Short(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Int(x), Value::Short(y)) => {
					match x.checked_add(y as i32) {
						Some(nb) => Ok(Value::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Float(x), Value::Short(y)) => {
					match checked_add_float(x, y as f32) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Double(x), Value::Short(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Int
				(Value::Char(x), Value::Int(y)) => {
					match y.checked_add(x as i32) {
						Some(nb) => Ok(Value::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Short(x), Value::Int(y)) => {
					match y.checked_add(x as i32) {
						Some(nb) => Ok(Value::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Int(x), Value::Int(y)) => {
					match x.checked_add(y) {
						Some(nb) => Ok(Value::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Value::Float(x), Value::Int(y)) => {
					match checked_add_float(x, y as f32) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Double(x), Value::Int(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Float
				(Value::Char(x), Value::Float(y)) => {
					match checked_add_float(x as f32, y) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Short(x), Value::Float(y)) => {
					match checked_add_float(x as f32, y) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Int(x), Value::Float(y)) => {
					match checked_add_float(x as f32, y) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Float(x), Value::Float(y)) => {
					match checked_add_float(x, y) {
						Ok(nb) => Ok(Value::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Double(x), Value::Float(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Double
				(Value::Char(x), Value::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Short(x), Value::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Int(x), Value::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Float(x), Value::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Value::Double(x), Value::Double(y)) => {
					match checked_add_float(x, y) {
						Ok(nb) => Ok(Value::Double(nb)),
						Err(e) => Err(e),
					}
				}
			}
		}
	}
}
