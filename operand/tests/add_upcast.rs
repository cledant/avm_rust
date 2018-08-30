extern crate operand;

use operand::value::Type;

//From Char
#[test]
fn operand_add_upcast_ok_1() {
	assert_eq!(Ok(Type::Short(75)), Type::Short(50) + Type::Char(25));
}

#[test]
fn operand_add_upcast_ok_2() {
	assert_eq!(Ok(Type::Int(75)), Type::Int(50) + Type::Char(25));
}

#[test]
fn operand_add_upcast_ok_3() {
	assert_eq!(Ok(Type::Float(75.00)), Type::Float(50.00) + Type::Char(25));
}

#[test]
fn operand_add_upcast_ok_4() {
	assert_eq!(Ok(Type::Double(75.00)), Type::Double(50.00) + Type::Char(25));
}

//From Short
#[test]
fn operand_add_upcast_ok_5() {
	assert_eq!(Ok(Type::Int(75)), Type::Int(50) + Type::Short(25));
}

#[test]
fn operand_add_upcast_ok_6() {
	assert_eq!(Ok(Type::Float(75.00)), Type::Float(50.00) + Type::Short(25));
}

#[test]
fn operand_add_upcast_ok_7() {
	assert_eq!(Ok(Type::Double(75.00)), Type::Double(50.00) + Type::Short(25));
}

//From Int
#[test]
fn operand_add_upcast_ok_8() {
	assert_eq!(Ok(Type::Float(75.00)), Type::Int(50) + Type::Float(25.00));
}

#[test]
fn operand_add_upcast_ok_9() {
	assert_eq!(Ok(Type::Double(75.00)), Type::Int(50) + Type::Double(25.00));
}

//From Float
#[test]
fn operand_add_upcast_ok_10() {
	assert_eq!(Ok(Type::Double(75.00)), Type::Float(50.00) + Type::Double(25.00));
}
