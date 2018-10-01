extern crate avm_lib;

use avm_lib::instruction;
use avm_lib::parser::Token;
use avm_lib::stack;
use avm_lib::stack::ExecState;
use avm_lib::value::Type;

#[test]
fn stack_no_exit() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    //Test
    assert_eq!(
        ExecState::Error(stack::ERR_NO_EXIT_INST),
        stack::run(&mut vec_tok)
    )
}

#[test]
fn stack_push() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(42)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_failed_pop() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: None,
        inst: Some(instruction::pop),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(
        ExecState::Error(instruction::ERR_EMPTY_STACK),
        stack::run(&mut vec_tok)
    );
}

#[test]
fn stack_failed_assert_1() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(42)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(42)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(
        ExecState::Error(instruction::ERR_FAILED_ASSERT),
        stack::run(&mut vec_tok)
    );
}

#[test]
fn stack_failed_assert_2() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(42)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Char(52)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(
        ExecState::Error(instruction::ERR_FAILED_ASSERT),
        stack::run(&mut vec_tok)
    );
}

#[test]
fn stack_dump() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Int(30)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Float(21.21)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Double(42.42)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::dump),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_pop() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::pop),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_add_assert() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::add),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(30)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_sub_assert() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::sub),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(-10)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_mul_assert() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::mul),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(200)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_div_assert() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::div),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(0)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_rem_assert() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(20)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::rem),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Short(10)),
        inst: Some(instruction::assert),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_print_1() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(82)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::print),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Char(101)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::print),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_print_2() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Char(10)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::print),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: Some(Type::Char(127)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::print),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(ExecState::Stop, stack::run(&mut vec_tok));
}

#[test]
fn stack_failed_print() {
    //Init
    let mut vec_tok: Vec<Token> = Vec::new();
    let tok = Token {
        val: Some(Type::Short(42)),
        inst: Some(instruction::push),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::print),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    let tok = Token {
        val: None,
        inst: Some(instruction::exit),
        line: String::from("empty"),
        line_number: 0,
        vec_error: vec![],
    };
    vec_tok.push(tok);
    //Test
    assert_eq!(
        ExecState::Error(instruction::ERR_NOT_CHAR_TYPE),
        stack::run(&mut vec_tok)
    );
}
