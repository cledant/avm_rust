pub mod operand {

use std::process;
use std::ops::Add;

	pub enum Value {
		Char(i8),
		Short(i16),
		Int(i32),
		Float(f32),
		Double(f64),
	}	

	fn overflow_exit() -> Value {
		println!("Overflow detected ! Exiting !");
		process::exit(1);
	}

	fn checked_f32_add (x : f32, y : f32) -> Value {
		let val = x + y;
		if val.is_finite() {
			Value::Float(val)
		} else {
			overflow_exit()
		}
	}

	fn checked_f64_add (x : f64, y : f64) -> Value {
		let val = x + y;
		if val.is_finite() {
			Value::Double(val)
		} else {
			overflow_exit()
		}
	}

	impl Add for Value {
		type Output = Value;

		fn add(self, other : Value) -> Value {
			match (self, other) {
				//Char
				(Value::Char(x), Value::Char(y)) => { 
					let val = x.checked_add(y);
					match val {
						Some(x) => Value::Char(x),
						None => { overflow_exit() },
					}
				}
				(Value::Short(x), Value::Char(y)) => { 
					let val = x.checked_add(y as i16);
					match val {
						Some(x) => Value::Short(x),
						None => { overflow_exit() },
					}
				}
				(Value::Int(x), Value::Char(y)) => { 
					let val = x.checked_add(y as i32);
					match val {
						Some(x) => Value::Int(x),
						None => { overflow_exit() },
					}
				}
				(Value::Float(x), Value::Char(y)) => {
					checked_f32_add(x, y as f32)
				}
				(Value::Double(x), Value::Char(y)) => {
					checked_f64_add(x, y as f64)
				}
				//Short
				(Value::Char(x), Value::Short(y)) => { 
					let val = y.checked_add(x as i16);
					match val {
						Some(x) => Value::Short(x),
						None => { overflow_exit() },
					}
				}
				(Value::Short(x), Value::Short(y)) => { 
					let val = x.checked_add(y);
					match val {
						Some(x) => Value::Short(x),
						None => { overflow_exit() },
					}
				}
				(Value::Int(x), Value::Short(y)) => { 
					let val = x.checked_add(y as i32);
					match val {
						Some(x) => Value::Int(x),
						None => { overflow_exit() },
					}
				}
				(Value::Float(x), Value::Short(y)) => {
					checked_f32_add(x, y as f32)
				}
				(Value::Double(x), Value::Short(y)) => {
					checked_f64_add(x, y as f64)
				}
				//Int
				(Value::Char(x), Value::Int(y)) => { 
					let val = y.checked_add(x as i32);
					match val {
						Some(x) => Value::Int(x),
						None => { overflow_exit() },
					}
				}
				(Value::Short(x), Value::Int(y)) => {
 					let val = y.checked_add(x as i32);
					match val {
						Some(x) => Value::Int(x),
						None => { overflow_exit() },
					}
				}
				(Value::Int(x), Value::Int(y)) => { 
					let val = x.checked_add(y);
					match val {
						Some(x) => Value::Int(x),
						None => { overflow_exit() },
					}
				}
				(Value::Float(x), Value::Int(y)) => {
					checked_f32_add(x, y as f32)
				}
				(Value::Double(x), Value::Int(y)) => {
					checked_f64_add(x, y as f64)
				}
				//Float
				(Value::Char(x), Value::Float(y)) => { 
					checked_f32_add(x as f32, y)
				}
				(Value::Short(x), Value::Float(y)) => {
					checked_f32_add(x as f32, y)
				}
				(Value::Int(x), Value::Float(y)) => { 
					checked_f32_add(x as f32, y)
				}
				(Value::Float(x), Value::Float(y)) => {
					checked_f32_add(x, y)
				}
				(Value::Double(x), Value::Float(y)) => {
					checked_f64_add(x, y as f64)
				}
				//Double
				(Value::Char(x), Value::Double(y)) => { 
					checked_f64_add(x as f64, y)
				}
				(Value::Short(x), Value::Double(y)) => {
					checked_f64_add(x as f64, y)
				}
				(Value::Int(x), Value::Double(y)) => { 
					checked_f64_add(x as f64, y)
				}
				(Value::Float(x), Value::Double(y)) => {
					checked_f64_add(x as f64, y)
				}
				(Value::Double(x), Value::Double(y)) => {
					checked_f64_add(x, y)
				}
			}
		}
	}
}
