pub mod operand {

use std::process;
use std::ops::Add;
use std::convert::From;

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

	impl Add for Value {
		type Output = Value;

		fn add(self, other : Value) -> Value {
			match (self, other) {
				(Value::Char(x), Value::Char(y)) => { 
					let val = x.checked_add(y);
					match val {
						Some(x) => Value::Char(x),
						None => { overflow_exit() },
					}
				}
				(Value::Short(x), Value::Char(y)) => { 
					let val = x.checked_add(From::from(y));
					match val {
						Some(x) => Value::Short(x),
						None => { overflow_exit() },
					}
				}
				(Value::Int(x), Value::Char(y)) => { 
					let val = x.checked_add(From::from(y));
					match val {
						Some(x) => Value::Int(x),
						None => { overflow_exit() },
					}
				}
				(Value::Float(x), Value::Char(y)) => {
					let v : f32 = From::from(y);
					let val : f32 = x + v;
					if val.is_finite() {
						Value::Float(x)
					} else {
						overflow_exit()
					}
				}
				(Value::Double(x), Value::Char(y)) => {
 					let v : f64 = From::from(y);
					let val : f64 = x + v;
					if val.is_finite() {
						Value::Double(x)
					} else {
						overflow_exit()
					}
				}
			}
		}
	}
}
