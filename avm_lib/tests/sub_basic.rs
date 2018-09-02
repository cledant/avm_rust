extern crate avm_lib;

use avm_lib::value;
use avm_lib::value::Type;

//Char
#[test]
fn operand_sub_char_ok() {
	assert_eq!(Ok(Type::Char(25)), Type::Char(50) - Type::Char(25));
}

#[test]
fn operand_sub_char_failed() {
	assert_eq!(Err(value::ERR_OVERFLOW), Type::Char(-100) - Type::Char(100));
}

//Short
#[test]
fn operand_sub_short_ok() {
	assert_eq!(Ok(Type::Short(-28000)), Type::Short(2000) - Type::Short(30000));
}

#[test]
fn operand_sub_short_failed() {
	assert_eq!(Err(value::ERR_OVERFLOW), Type::Short(-5000) - Type::Short(30000));
}

//Int
#[test]
fn operand_sub_int_ok() {
	assert_eq!(Ok(Type::Int(0)), Type::Int(500000) - Type::Int(500000));
}

#[test]
fn operand_sub_int_failed() {
	assert_eq!(Err(value::ERR_OVERFLOW), Type::Int(-2000000000) - Type::Int(2000000000));
}

//Float
#[test]
fn operand_sub_float_ok() {
	assert_eq!(Ok(Type::Float(7500.5)), Type::Float(10000.5) - Type::Float(2500.0));
}

#[test]
fn operand_sub_float_failed() {
	assert_eq!(Err(value::ERR_OVERFLOW), Type::Float(std::f32::MIN) - Type::Float(std::f32::MAX));
}

//Double
#[test]
fn operand_sub_double_ok() {
	assert_eq!(Ok(Type::Double(7500.5)), Type::Double(10000.5) - Type::Double(2500.0));
}

#[test]
fn operand_sub_double_failed() {
	assert_eq!(Err(value::ERR_OVERFLOW), Type::Double(std::f64::MIN) - Type::Double(std::f64::MAX));
}
