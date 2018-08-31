extern crate operand;

use operand::value::Type;

//From Char
#[test]
fn operand_mul_upcast_ok_1() {
	assert_eq!(Ok(Type::Short(125)), Type::Short(5) * Type::Char(25));
}

#[test]
fn operand_mul_upcast_ok_2() {
	assert_eq!(Ok(Type::Int(125)), Type::Int(5) * Type::Char(25));
}

#[test]
fn operand_mul_upcast_ok_3() {
	assert_eq!(Ok(Type::Float(125.00)), Type::Float(5.00) * Type::Char(25));
}

#[test]
fn operand_mul_upcast_ok_4() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Double(5.00) * Type::Char(25));
}

#[test]
fn operand_mul_upcast_ok_5() {
	assert_eq!(Ok(Type::Short(125)), Type::Char(25) * Type::Short(5));
}

#[test]
fn operand_mul_upcast_ok_6() {
	assert_eq!(Ok(Type::Int(125)), Type::Char(25) * Type::Int(5));
}

#[test]
fn operand_mul_upcast_ok_7() {
	assert_eq!(Ok(Type::Float(125.00)), Type::Char(25) *Type::Float(5.00));
}

#[test]
fn operand_mul_upcast_ok_8() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Char(25) *Type::Double(5.00));
}

//From Short
#[test]
fn operand_mul_upcast_ok_9() {
	assert_eq!(Ok(Type::Int(125)), Type::Int(5) * Type::Short(25));
}

#[test]
fn operand_mul_upcast_ok_10() {
	assert_eq!(Ok(Type::Float(125.00)), Type::Float(5.00) * Type::Short(25));
}

#[test]
fn operand_mul_upcast_ok_11() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Double(5.00) * Type::Short(25));
}

#[test]
fn operand_mul_upcast_ok_12() {
	assert_eq!(Ok(Type::Int(125)), Type::Short(25) * Type::Int(5));
}

#[test]
fn operand_mul_upcast_ok_13() {
	assert_eq!(Ok(Type::Float(125.00)), Type::Short(25) * Type::Float(5.00));
}

#[test]
fn operand_mul_upcast_ok_14() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Short(25) * Type::Double(5.00));
}

//From Int
#[test]
fn operand_mul_upcast_ok_15() {
	assert_eq!(Ok(Type::Float(125.00)), Type::Int(5) * Type::Float(25.00));
}

#[test]
fn operand_mul_upcast_ok_16() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Int(5) * Type::Double(25.00));
}

#[test]
fn operand_mul_upcast_ok_17() {
	assert_eq!(Ok(Type::Float(125.00)), Type::Float(25.00) * Type::Int(5));
}

#[test]
fn operand_mul_upcast_ok_18() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Double(25.00) * Type::Int(5));
}

//From Float
#[test]
fn operand_mul_upcast_ok_19() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Float(5.00) * Type::Double(25.00));
}

#[test]
fn operand_mul_upcast_ok_20() {
	assert_eq!(Ok(Type::Double(125.00)), Type::Double(25.00) * Type::Float(5.00));
}
