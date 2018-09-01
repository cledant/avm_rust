extern crate operand;

use operand::value::Type;

//Char
#[test]
fn operand_rem_char_ok() {
	assert_eq!(Ok(Type::Char(0)), Type::Char(50) % Type::Char(2));
}

#[test]
fn operand_rem_0_char_failed() {
	assert_eq!(Err(operand::value::ERR_MOD_0), Type::Char(50) % Type::Char(0));
}

//Short
#[test]
fn operand_rem_short_ok() {
	assert_eq!(Ok(Type::Short(0)), Type::Short(32000) % Type::Short(2));
}

#[test]
fn operand_rem_0_short_failed() {
	assert_eq!(Err(operand::value::ERR_MOD_0), Type::Short(10000) % Type::Short(0));
}

//Int
#[test]
fn operand_rem_int_ok() {
	assert_eq!(Ok(Type::Int(50)), Type::Int(50) % Type::Int(500000));
}

#[test]
fn operand_rem_0_int_failed() {
	assert_eq!(Err(operand::value::ERR_MOD_0), Type::Int(2000000000) % Type::Int(0));
}

//Float
#[test]
fn operand_rem_float_ok() {
	assert_eq!(Ok(Type::Float(1.0)), Type::Float(20001.0) % Type::Float(2.0));
}

#[test]
fn operand_rem_0_float_failed() {
	assert_eq!(Err(operand::value::ERR_NAN), Type::Float(std::f32::MAX) % Type::Float(0.0));
}

//Double
#[test]
fn operand_rem_double_ok() {
	assert_eq!(Ok(Type::Double(1.0)), Type::Double(20001.0) % Type::Double(2.0));
}

#[test]
fn operand_rem_0_double_failed() {
	assert_eq!(Err(operand::value::ERR_NAN), Type::Double(10000.0) % Type::Double(0.0));
}
