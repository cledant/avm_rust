pub mod value {

extern crate num;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::num::FpCategory;

	#[derive(Debug, PartialEq)]
	pub enum Type {
		Char(i8),
		Short(i16),
		Int(i32),
		Float(f32),
		Double(f64),
	}	

	enum RetType {
		Char,
		Short,
		Int,
		Float,
		Double,
	}

	pub static ERR_OVERFLOW : &str = "Overflow detected";
	pub static ERR_IMPOSSIBLE : &str = "Impossible case";

	//Add
	#[inline]
	fn checked_add_float<T> (x : T, y : T, ret : RetType) -> Result<Type, &'static str> 
	where T : num::Float + num::cast::ToPrimitive
	{
		let val = x + y;
		match (val.classify(), ret) {
			(FpCategory::Subnormal, _) => Err(ERR_OVERFLOW),
			(FpCategory::Infinite, _) => Err(ERR_OVERFLOW),
			(FpCategory::Nan, _) => Err(ERR_OVERFLOW),
			(_, RetType::Float) => Ok(Type::Float(val.to_f32().unwrap())),
			(_, RetType::Double) => Ok(Type::Double(val.to_f64().unwrap())),
			(_, _) => Err(ERR_IMPOSSIBLE),
		}
	}

	#[inline]
	fn checked_add_integer<T> (x : T, y : T, ret : RetType) -> Result<Type, &'static str> 
	where T : num::CheckedAdd + num::cast::ToPrimitive
	{
		match (x.checked_add(&y), ret) {
			(Some(nb) , RetType::Char) => Ok(Type::Char(nb.to_i8().unwrap())),
			(Some(nb) , RetType::Short) => Ok(Type::Short(nb.to_i16().unwrap())),
			(Some(nb) , RetType::Int) => Ok(Type::Int(nb.to_i32().unwrap())),
			(Some(_) , _) => Err(ERR_IMPOSSIBLE),
			(None, _) => Err(ERR_OVERFLOW),
		}	
	}

	impl Add for Type {
		type Output = Result<Type, &'static str>;

		fn add(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_add_integer(x, y, RetType::Char),
				(Type::Short(x), Type::Char(y)) => checked_add_integer(x, y as i16, RetType::Short),
				(Type::Int(x), Type::Char(y)) => checked_add_integer(x, y as i32, RetType::Int),
				(Type::Float(x), Type::Char(y)) => checked_add_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Char(y)) => checked_add_float(x, y as f64, RetType::Double),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_add_integer(x as i16, y, RetType::Short),
				(Type::Short(x), Type::Short(y)) => checked_add_integer(x, y, RetType::Short),
				(Type::Int(x), Type::Short(y)) => checked_add_integer(x, y as i32, RetType::Int),
				(Type::Float(x), Type::Short(y)) => checked_add_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Short(y)) => checked_add_float(x, y as f64, RetType::Double),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_add_integer(x as i32, y, RetType::Int),
				(Type::Short(x), Type::Int(y)) => checked_add_integer(x as i32, y, RetType::Int),
				(Type::Int(x), Type::Int(y)) => checked_add_integer(x, y, RetType::Int),
				(Type::Float(x), Type::Int(y)) => checked_add_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Int(y)) => checked_add_float(x, y as f64, RetType::Double),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_add_float(x as f32, y, RetType::Float),
				(Type::Short(x), Type::Float(y)) => checked_add_float(x as f32, y, RetType::Float),
				(Type::Int(x), Type::Float(y)) => checked_add_float(x as f32, y, RetType::Float),
				(Type::Float(x), Type::Float(y)) => checked_add_float(x, y, RetType::Float),
				(Type::Double(x), Type::Float(y)) => checked_add_float(x, y as f64, RetType::Double),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_add_float(x as f64, y, RetType::Double),
				(Type::Short(x), Type::Double(y)) => checked_add_float(x as f64, y, RetType::Double),
				(Type::Int(x), Type::Double(y)) => checked_add_float(x as f64, y, RetType::Double),
				(Type::Float(x), Type::Double(y)) => checked_add_float(x as f64, y, RetType::Double),
				(Type::Double(x), Type::Double(y)) => checked_add_float(x, y, RetType::Double),
			}
		}
	}

	//Sub Operator
	#[inline]
	fn checked_sub_float<T> (x : T, y : T, ret : RetType) -> Result<Type, &'static str> 
	where T : num::Float + num::cast::ToPrimitive
	{
		let val = x - y;
		match (val.classify(), ret) {
			(FpCategory::Subnormal, _) => Err(ERR_OVERFLOW),
			(FpCategory::Infinite, _) => Err(ERR_OVERFLOW),
			(FpCategory::Nan, _) => Err(ERR_OVERFLOW),
			(_, RetType::Float) => Ok(Type::Float(val.to_f32().unwrap())),
			(_, RetType::Double) => Ok(Type::Double(val.to_f64().unwrap())),
			(_, _) => Err(ERR_IMPOSSIBLE),
		}
	}

	#[inline]
	fn checked_sub_integer<T> (x : T, y : T, ret : RetType) -> Result<Type, &'static str> 
	where T : num::CheckedSub + num::cast::ToPrimitive
	{
		match (x.checked_sub(&y), ret) {
			(Some(nb) , RetType::Char) => Ok(Type::Char(nb.to_i8().unwrap())),
			(Some(nb) , RetType::Short) => Ok(Type::Short(nb.to_i16().unwrap())),
			(Some(nb) , RetType::Int) => Ok(Type::Int(nb.to_i32().unwrap())),
			(Some(_) , _) => Err(ERR_IMPOSSIBLE),
			(None, _) => Err(ERR_OVERFLOW),
		}	
	}

	impl Sub for Type {
		type Output = Result<Type, &'static str>;

		fn sub(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_sub_integer(x, y, RetType::Char),
				(Type::Short(x), Type::Char(y)) => checked_sub_integer(x, y as i16, RetType::Short),
				(Type::Int(x), Type::Char(y)) => checked_sub_integer(x, y as i32, RetType::Int),
				(Type::Float(x), Type::Char(y)) => checked_sub_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Char(y)) => checked_sub_float(x, y as f64, RetType::Double),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_sub_integer(x as i16, y, RetType::Short),
				(Type::Short(x), Type::Short(y)) => checked_sub_integer(x, y, RetType::Short),
				(Type::Int(x), Type::Short(y)) => checked_sub_integer(x, y as i32, RetType::Int),
				(Type::Float(x), Type::Short(y)) => checked_sub_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Short(y)) => checked_sub_float(x, y as f64, RetType::Double),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_sub_integer(x as i32, y, RetType::Int),
				(Type::Short(x), Type::Int(y)) => checked_sub_integer(x as i32, y, RetType::Int),
				(Type::Int(x), Type::Int(y)) => checked_sub_integer(x, y, RetType::Int),
				(Type::Float(x), Type::Int(y)) => checked_sub_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Int(y)) => checked_sub_float(x, y as f64, RetType::Double),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_sub_float(x as f32, y, RetType::Float),
				(Type::Short(x), Type::Float(y)) => checked_sub_float(x as f32, y, RetType::Float),
				(Type::Int(x), Type::Float(y)) => checked_sub_float(x as f32, y, RetType::Float),
				(Type::Float(x), Type::Float(y)) => checked_sub_float(x, y, RetType::Float),
				(Type::Double(x), Type::Float(y)) => checked_sub_float(x, y as f64, RetType::Double),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_sub_float(x as f64, y, RetType::Double),
				(Type::Short(x), Type::Double(y)) => checked_sub_float(x as f64, y, RetType::Double),
				(Type::Int(x), Type::Double(y)) => checked_sub_float(x as f64, y, RetType::Double),
				(Type::Float(x), Type::Double(y)) => checked_sub_float(x as f64, y, RetType::Double),
				(Type::Double(x), Type::Double(y)) => checked_sub_float(x, y, RetType::Double),
			}
		}
	}

	//Mult Operator
	#[inline]
	fn checked_mul_float<T> (x : T, y : T, ret : RetType) -> Result<Type, &'static str> 
	where T : num::Float + num::cast::ToPrimitive
	{
		let val = x * y;
		match (val.classify(), ret) {
			(FpCategory::Subnormal, _) => Err(ERR_OVERFLOW),
			(FpCategory::Infinite, _) => Err(ERR_OVERFLOW),
			(FpCategory::Nan, _) => Err(ERR_OVERFLOW),
			(_, RetType::Float) => Ok(Type::Float(val.to_f32().unwrap())),
			(_, RetType::Double) => Ok(Type::Double(val.to_f64().unwrap())),
			(_, _) => Err(ERR_IMPOSSIBLE),
		}
	}

	#[inline]
	fn checked_mul_integer<T> (x : T, y : T, ret : RetType) -> Result<Type, &'static str> 
	where T : num::CheckedMul + num::cast::ToPrimitive
	{
		match (x.checked_mul(&y), ret) {
			(Some(nb) , RetType::Char) => Ok(Type::Char(nb.to_i8().unwrap())),
			(Some(nb) , RetType::Short) => Ok(Type::Short(nb.to_i16().unwrap())),
			(Some(nb) , RetType::Int) => Ok(Type::Int(nb.to_i32().unwrap())),
			(Some(_) , _) => Err(ERR_IMPOSSIBLE),
			(None, _) => Err(ERR_OVERFLOW),
		}	
	}

	impl Mul for Type {
		type Output = Result<Type, &'static str>;

		fn mul(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_mul_integer(x, y, RetType::Char),
				(Type::Short(x), Type::Char(y)) => checked_mul_integer(x, y as i16, RetType::Short),
				(Type::Int(x), Type::Char(y)) => checked_mul_integer(x, y as i32, RetType::Int),
				(Type::Float(x), Type::Char(y)) => checked_mul_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Char(y)) => checked_mul_float(x, y as f64, RetType::Double),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_mul_integer(x as i16, y, RetType::Short),
				(Type::Short(x), Type::Short(y)) => checked_mul_integer(x, y, RetType::Short),
				(Type::Int(x), Type::Short(y)) => checked_mul_integer(x, y as i32, RetType::Int),
				(Type::Float(x), Type::Short(y)) => checked_mul_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Short(y)) => checked_mul_float(x, y as f64, RetType::Double),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_mul_integer(x as i32, y, RetType::Int),
				(Type::Short(x), Type::Int(y)) => checked_mul_integer(x as i32, y, RetType::Int),
				(Type::Int(x), Type::Int(y)) => checked_mul_integer(x, y, RetType::Int),
				(Type::Float(x), Type::Int(y)) => checked_mul_float(x, y as f32, RetType::Float),
				(Type::Double(x), Type::Int(y)) => checked_mul_float(x, y as f64, RetType::Double),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_mul_float(x as f32, y, RetType::Float),
				(Type::Short(x), Type::Float(y)) => checked_mul_float(x as f32, y, RetType::Float),
				(Type::Int(x), Type::Float(y)) => checked_mul_float(x as f32, y, RetType::Float),
				(Type::Float(x), Type::Float(y)) => checked_mul_float(x, y, RetType::Float),
				(Type::Double(x), Type::Float(y)) => checked_mul_float(x, y as f64, RetType::Double),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_mul_float(x as f64, y, RetType::Double),
				(Type::Short(x), Type::Double(y)) => checked_mul_float(x as f64, y, RetType::Double),
				(Type::Int(x), Type::Double(y)) => checked_mul_float(x as f64, y, RetType::Double),
				(Type::Float(x), Type::Double(y)) => checked_mul_float(x as f64, y, RetType::Double),
				(Type::Double(x), Type::Double(y)) => checked_mul_float(x, y, RetType::Double),
			}
		}
	}
}
