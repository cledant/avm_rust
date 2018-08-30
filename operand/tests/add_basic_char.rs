extern crate operand;

use operand::value::Type;

#[test]
fn operand_add_char_ok_1() {
	assert_eq!(Ok(Type::Char(75)), Type::Char(50) + Type::Char(25));
}

#[test]
fn operand_add_char_ok_2() {
	assert_eq!(Ok(Type::Char(-25)), Type::Char(-50) + Type::Char(25));
}

#[test]
fn operand_add_char_ok_3() {
	assert_eq!(Ok(Type::Char(-75)), Type::Char(-50) + Type::Char(-25));
}

#[test]
fn operand_add_char_ok_4() {
	assert_eq!(Ok(Type::Char(25)), Type::Char(50) + Type::Char(-25));
}

#[test]
fn operand_add_char_failed_1() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Char(50) + Type::Char(100));
}

#[test]
fn operand_add_char_failed_2() {
	assert_eq!(Err(operand::value::ERR_OVERFLOW), Type::Char(-50) + Type::Char(-100));
}
