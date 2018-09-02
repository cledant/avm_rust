extern crate num;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;
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

	enum OpType {
		Add,
		Sub,
		Mul,
		Div,
		Rem,
	}

	pub static ERR_OVERFLOW : &str = "Overflow detected";
	pub static ERR_IMPOSSIBLE : &str = "Impossible case";
	pub static ERR_DIV_0 : &str = "Divided by 0 or too small number";
	pub static ERR_MOD_0 : &str = "Modulo by 0 or too small number";
	pub static ERR_NAN : &str = "Float result is not a number";

	#[inline]
	fn checked_op_float<T> (x : T, y : T, ret : RetType, op : OpType) -> Result<Type, &'static str> 
	where T : num::Float + num::cast::ToPrimitive
	{
		let val;
		match op {
			OpType::Add => {val = x + y}
			OpType::Sub => {val = x - y}
			OpType::Mul => {val = x * y}
			OpType::Div => {val = x / y}
			OpType::Rem => {val = x % y}
		}
		match (val.classify(), ret, op) {
			(FpCategory::Subnormal, _, _) => Err(ERR_OVERFLOW),
			(FpCategory::Infinite, _, OpType::Div) => Err(ERR_DIV_0),
			(FpCategory::Infinite, _, OpType::Rem) => Err(ERR_MOD_0),
			(FpCategory::Infinite, _, _) => Err(ERR_OVERFLOW),
			(FpCategory::Nan, _, _) => Err(ERR_NAN),
			(_, RetType::Float, _) => Ok(Type::Float(val.to_f32().unwrap())),
			(_, RetType::Double, _) => Ok(Type::Double(val.to_f64().unwrap())),
			(_, _, _) => Err(ERR_IMPOSSIBLE),
		}
	}

	#[inline]
	fn checked_op_integer<T> (x : T, y : T, ret : RetType, op : OpType) -> Result<Type, &'static str> 
	where T : num::CheckedAdd + num::CheckedSub + num::CheckedMul + num::CheckedDiv +
			num::traits::CheckedRem + num::cast::ToPrimitive
	{
		let val;
		match op {
			OpType::Add => {val = x.checked_add(&y)}
			OpType::Sub => {val = x.checked_sub(&y)}
			OpType::Mul => {val = x.checked_mul(&y)}
			OpType::Div => {val = x.checked_div(&y)}
			OpType::Rem => {val = x.checked_rem(&y)}
		}
		match (val, ret, op) {
			(Some(nb) , RetType::Char, _) => Ok(Type::Char(nb.to_i8().unwrap())),
			(Some(nb) , RetType::Short,_) => Ok(Type::Short(nb.to_i16().unwrap())),
			(Some(nb) , RetType::Int, _) => Ok(Type::Int(nb.to_i32().unwrap())),
			(Some(_) , _, _) => Err(ERR_IMPOSSIBLE),
			(None, _, OpType::Div) => Err(ERR_DIV_0),
			(None, _, OpType::Rem) => Err(ERR_MOD_0),
			(None, _, _) => Err(ERR_OVERFLOW),
		}	
	}

	impl Add for Type {
		type Output = Result<Type, &'static str>;

		fn add(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_op_integer(x, y, RetType::Char, OpType::Add),
				(Type::Short(x), Type::Char(y)) => checked_op_integer(x, y as i16, RetType::Short, OpType::Add),
				(Type::Int(x), Type::Char(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Add),
				(Type::Float(x), Type::Char(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Add),
				(Type::Double(x), Type::Char(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Add),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_op_integer(x as i16, y, RetType::Short, OpType::Add),
				(Type::Short(x), Type::Short(y)) => checked_op_integer(x, y, RetType::Short, OpType::Add),
				(Type::Int(x), Type::Short(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Add),
				(Type::Float(x), Type::Short(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Add),
				(Type::Double(x), Type::Short(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Add),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Add),
				(Type::Short(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Add),
				(Type::Int(x), Type::Int(y)) => checked_op_integer(x, y, RetType::Int, OpType::Add),
				(Type::Float(x), Type::Int(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Add),
				(Type::Double(x), Type::Int(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Add),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Add),
				(Type::Short(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Add),
				(Type::Int(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Add),
				(Type::Float(x), Type::Float(y)) => checked_op_float(x, y, RetType::Float, OpType::Add),
				(Type::Double(x), Type::Float(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Add),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Add),
				(Type::Short(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Add),
				(Type::Int(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Add),
				(Type::Float(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Add),
				(Type::Double(x), Type::Double(y)) => checked_op_float(x, y, RetType::Double, OpType::Add),
			}
		}
	}

	impl Sub for Type {
		type Output = Result<Type, &'static str>;

		fn sub(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_op_integer(x, y, RetType::Char, OpType::Sub),
				(Type::Short(x), Type::Char(y)) => checked_op_integer(x, y as i16, RetType::Short, OpType::Sub),
				(Type::Int(x), Type::Char(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Sub),
				(Type::Float(x), Type::Char(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Sub),
				(Type::Double(x), Type::Char(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Sub),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_op_integer(x as i16, y, RetType::Short, OpType::Sub),
				(Type::Short(x), Type::Short(y)) => checked_op_integer(x, y, RetType::Short, OpType::Sub),
				(Type::Int(x), Type::Short(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Sub),
				(Type::Float(x), Type::Short(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Sub),
				(Type::Double(x), Type::Short(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Sub),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Sub),
				(Type::Short(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Sub),
				(Type::Int(x), Type::Int(y)) => checked_op_integer(x, y, RetType::Int, OpType::Sub),
				(Type::Float(x), Type::Int(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Sub),
				(Type::Double(x), Type::Int(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Sub),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Sub),
				(Type::Short(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Sub),
				(Type::Int(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Sub),
				(Type::Float(x), Type::Float(y)) => checked_op_float(x, y, RetType::Float, OpType::Sub),
				(Type::Double(x), Type::Float(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Sub),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Sub),
				(Type::Short(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Sub),
				(Type::Int(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Sub),
				(Type::Float(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Sub),
				(Type::Double(x), Type::Double(y)) => checked_op_float(x, y, RetType::Double, OpType::Sub),
			}
		}
	}

	impl Mul for Type {
		type Output = Result<Type, &'static str>;

		fn mul(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_op_integer(x, y, RetType::Char, OpType::Mul),
				(Type::Short(x), Type::Char(y)) => checked_op_integer(x, y as i16, RetType::Short, OpType::Mul),
				(Type::Int(x), Type::Char(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Mul),
				(Type::Float(x), Type::Char(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Mul),
				(Type::Double(x), Type::Char(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Mul),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_op_integer(x as i16, y, RetType::Short, OpType::Mul),
				(Type::Short(x), Type::Short(y)) => checked_op_integer(x, y, RetType::Short, OpType::Mul),
				(Type::Int(x), Type::Short(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Mul),
				(Type::Float(x), Type::Short(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Mul),
				(Type::Double(x), Type::Short(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Mul),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Mul),
				(Type::Short(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Mul),
				(Type::Int(x), Type::Int(y)) => checked_op_integer(x, y, RetType::Int, OpType::Mul),
				(Type::Float(x), Type::Int(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Mul),
				(Type::Double(x), Type::Int(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Mul),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Mul),
				(Type::Short(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Mul),
				(Type::Int(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Mul),
				(Type::Float(x), Type::Float(y)) => checked_op_float(x, y, RetType::Float, OpType::Mul),
				(Type::Double(x), Type::Float(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Mul),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Mul),
				(Type::Short(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Mul),
				(Type::Int(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Mul),
				(Type::Float(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Mul),
				(Type::Double(x), Type::Double(y)) => checked_op_float(x, y, RetType::Double, OpType::Mul),
			}
		}
	}

	impl Div for Type {
		type Output = Result<Type, &'static str>;

		fn div(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_op_integer(x, y, RetType::Char, OpType::Div),
				(Type::Short(x), Type::Char(y)) => checked_op_integer(x, y as i16, RetType::Short, OpType::Div),
				(Type::Int(x), Type::Char(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Div),
				(Type::Float(x), Type::Char(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Div),
				(Type::Double(x), Type::Char(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Div),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_op_integer(x as i16, y, RetType::Short, OpType::Div),
				(Type::Short(x), Type::Short(y)) => checked_op_integer(x, y, RetType::Short, OpType::Div),
				(Type::Int(x), Type::Short(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Div),
				(Type::Float(x), Type::Short(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Div),
				(Type::Double(x), Type::Short(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Div),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Div),
				(Type::Short(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Div),
				(Type::Int(x), Type::Int(y)) => checked_op_integer(x, y, RetType::Int, OpType::Div),
				(Type::Float(x), Type::Int(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Div),
				(Type::Double(x), Type::Int(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Div),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Div),
				(Type::Short(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Div),
				(Type::Int(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Div),
				(Type::Float(x), Type::Float(y)) => checked_op_float(x, y, RetType::Float, OpType::Div),
				(Type::Double(x), Type::Float(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Div),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Div),
				(Type::Short(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Div),
				(Type::Int(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Div),
				(Type::Float(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Div),
				(Type::Double(x), Type::Double(y)) => checked_op_float(x, y, RetType::Double, OpType::Div),
			}
		}
	}

	impl Rem for Type {
		type Output = Result<Type, &'static str>;

		fn rem(self, other : Type) -> Result<Type, &'static str> {
			match (self, other) {
				//Char
				(Type::Char(x), Type::Char(y)) => checked_op_integer(x, y, RetType::Char, OpType::Rem),
				(Type::Short(x), Type::Char(y)) => checked_op_integer(x, y as i16, RetType::Short, OpType::Rem),
				(Type::Int(x), Type::Char(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Rem),
				(Type::Float(x), Type::Char(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Rem),
				(Type::Double(x), Type::Char(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Rem),
				//Short
				(Type::Char(x), Type::Short(y)) => checked_op_integer(x as i16, y, RetType::Short, OpType::Rem),
				(Type::Short(x), Type::Short(y)) => checked_op_integer(x, y, RetType::Short, OpType::Rem),
				(Type::Int(x), Type::Short(y)) => checked_op_integer(x, y as i32, RetType::Int, OpType::Rem),
				(Type::Float(x), Type::Short(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Rem),
				(Type::Double(x), Type::Short(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Rem),
				//Int
				(Type::Char(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Rem),
				(Type::Short(x), Type::Int(y)) => checked_op_integer(x as i32, y, RetType::Int, OpType::Rem),
				(Type::Int(x), Type::Int(y)) => checked_op_integer(x, y, RetType::Int, OpType::Rem),
				(Type::Float(x), Type::Int(y)) => checked_op_float(x, y as f32, RetType::Float, OpType::Rem),
				(Type::Double(x), Type::Int(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Rem),
				//Float
				(Type::Char(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Rem),
				(Type::Short(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Rem),
				(Type::Int(x), Type::Float(y)) => checked_op_float(x as f32, y, RetType::Float, OpType::Rem),
				(Type::Float(x), Type::Float(y)) => checked_op_float(x, y, RetType::Float, OpType::Rem),
				(Type::Double(x), Type::Float(y)) => checked_op_float(x, y as f64, RetType::Double, OpType::Rem),
				//Double
				(Type::Char(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Rem),
				(Type::Short(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Rem),
				(Type::Int(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Rem),
				(Type::Float(x), Type::Double(y)) => checked_op_float(x as f64, y, RetType::Double, OpType::Rem),
				(Type::Double(x), Type::Double(y)) => checked_op_float(x, y, RetType::Double, OpType::Rem),
			}
		}
	}
