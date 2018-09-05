extern crate avm_lib;

use std::env;
use avm_lib::init;

fn main() {
	let args : Vec<String> = env::args().collect();
	init::run(&args);
}
