extern crate avm_lib;

use avm_lib::init;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    init::run(&args);
}
