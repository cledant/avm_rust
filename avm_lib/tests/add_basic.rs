extern crate avm_lib;

use avm_lib::value;
use avm_lib::value::Type;

//Char
#[test]
fn avm_lib_add_char_ok() {
    assert_eq!(Ok(Type::Char(75)), Type::Char(50) + Type::Char(25));
}

#[test]
fn avm_lib_add_char_failed() {
    assert_eq!(Err(value::ERR_OVERFLOW), Type::Char(50) + Type::Char(100));
}

//Short
#[test]
fn avm_lib_add_short_ok() {
    assert_eq!(
        Ok(Type::Short(32000)),
        Type::Short(2000) + Type::Short(30000)
    );
}

#[test]
fn avm_lib_add_short_failed() {
    assert_eq!(
        Err(value::ERR_OVERFLOW),
        Type::Short(5000) + Type::Short(30000)
    );
}

//Int
#[test]
fn avm_lib_add_int_ok() {
    assert_eq!(
        Ok(Type::Int(1000000)),
        Type::Int(500000) + Type::Int(500000)
    );
}

#[test]
fn avm_lib_add_int_failed() {
    assert_eq!(
        Err(value::ERR_OVERFLOW),
        Type::Int(2000000000) + Type::Int(2000000000)
    );
}

//Float
#[test]
fn avm_lib_add_float_ok() {
    assert_eq!(
        Ok(Type::Float(12500.5)),
        Type::Float(10000.5) + Type::Float(2500.0)
    );
}

#[test]
fn avm_lib_add_float_failed() {
    assert_eq!(
        Err(value::ERR_OVERFLOW),
        Type::Float(std::f32::MAX) + Type::Float(std::f32::MAX)
    );
}

//Double
#[test]
fn avm_lib_add_double_ok() {
    assert_eq!(
        Ok(Type::Double(12500.5)),
        Type::Double(10000.5) + Type::Double(2500.0)
    );
}

#[test]
fn avm_lib_add_double_failed() {
    assert_eq!(
        Err(value::ERR_OVERFLOW),
        Type::Double(std::f64::MAX) + Type::Double(std::f64::MAX)
    );
}
