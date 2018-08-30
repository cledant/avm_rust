pub mod value {

extern crate num;

use std::ops::Add;
use std::num::FpCategory;

	#[derive(Debug, PartialEq)]
	pub enum Type {
		Char(i8),
		Short(i16),
		Int(i32),
		Float(f32),
		Double(f64),
	}	

	pub static ERR_OVERFLOW : &str = "Overflow detected";

	#[inline]
	fn checked_add_float<T : num::Float> (x : T, y : T) -> Result<T, &'static str> {
		let val = x + y;
		match val.classify() {
			FpCategory::Subnormal => Err(ERR_OVERFLOW),
			FpCategory::Infinite => Err(ERR_OVERFLOW),
			FpCategory::Nan => Err(ERR_OVERFLOW),
			_ => Ok(val),
		}
	}

	impl Add for Type {
		type Output = Result<Type, &'static str>;

		fn add(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => {
					match x.checked_add(y) {
						Some(nb) => Ok(Type::Char(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Short(x), Type::Char(y)) => {
					match x.checked_add(y as i16) {
						Some(nb) => Ok(Type::Short(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Int(x), Type::Char(y)) => {
					match x.checked_add(y as i32) {
						Some(nb) => Ok(Type::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Float(x), Type::Char(y)) => {
					match checked_add_float(x, y as f32) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Double(x), Type::Char(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Short
				(Type::Char(x), Type::Short(y)) => {
					match y.checked_add(x as i16) {
						Some(nb) => Ok(Type::Short(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Short(x), Type::Short(y)) => {
					match x.checked_add(y) {
						Some(nb) => Ok(Type::Short(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Int(x), Type::Short(y)) => {
					match x.checked_add(y as i32) {
						Some(nb) => Ok(Type::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Float(x), Type::Short(y)) => {
					match checked_add_float(x, y as f32) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Double(x), Type::Short(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Int
				(Type::Char(x), Type::Int(y)) => {
					match y.checked_add(x as i32) {
						Some(nb) => Ok(Type::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Short(x), Type::Int(y)) => {
					match y.checked_add(x as i32) {
						Some(nb) => Ok(Type::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Int(x), Type::Int(y)) => {
					match x.checked_add(y) {
						Some(nb) => Ok(Type::Int(nb)),
						None => Err(ERR_OVERFLOW),
					}
				}
				(Type::Float(x), Type::Int(y)) => {
					match checked_add_float(x, y as f32) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Double(x), Type::Int(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Float
				(Type::Char(x), Type::Float(y)) => {
					match checked_add_float(x as f32, y) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Short(x), Type::Float(y)) => {
					match checked_add_float(x as f32, y) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Int(x), Type::Float(y)) => {
					match checked_add_float(x as f32, y) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Float(x), Type::Float(y)) => {
					match checked_add_float(x, y) {
						Ok(nb) => Ok(Type::Float(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Double(x), Type::Float(y)) => {
					match checked_add_float(x, y as f64) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				//Double
				(Type::Char(x), Type::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Short(x), Type::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Int(x), Type::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Float(x), Type::Double(y)) => {
					match checked_add_float(x as f64, y) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
				(Type::Double(x), Type::Double(y)) => {
					match checked_add_float(x, y) {
						Ok(nb) => Ok(Type::Double(nb)),
						Err(e) => Err(e),
					}
				}
			}
		}
	}
}
