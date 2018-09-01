extern crate operand;

use operand::value::Type;

//Char
#[test]
fn operand_mul_char_ok() {
	assert_eq!(Ok(Type::Char(100)), Type::Char(4) * Type::Char(25));
}

#[test]
fn operand_mul_char_failed() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Char(50) * Type::Char(100));
}

//Short
#[test]
fn operand_mul_short_ok() {
	assert_eq!(Ok(Type::Short(32000)), Type::Short(2000) * Type::Short(16));
}

#[test]
fn operand_mul_short_failed() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Short(5000) * Type::Short(30000));
}

//Int
#[test]
fn operand_mul_int_ok() {
	assert_eq!(Ok(Type::Int(1000000)), Type::Int(1000) * Type::Int(1000));
}

#[test]
fn operand_mul_int_failed() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Int(2000000000) * Type::Int(2000000000));
}

//Float
#[test]
fn operand_mul_float_ok() {
	assert_eq!(Ok(Type::Float(12500.0)), Type::Float(1.25) * Type::Float(10000.0));
}

#[test]
fn operand_mul_float_failed() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Float(std::f32::MAX) * Type::Float(std::f32::MAX));
}

//Double
#[test]
fn operand_mul_double_ok() {
	assert_eq!(Ok(Type::Double(12500.0)), Type::Double(1.25) * Type::Double(10000.0));
}

#[test]
fn operand_mul_double_failed() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Double(std::f64::MAX) * Type::Double(std::f64::MAX));
}
