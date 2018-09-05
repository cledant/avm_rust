extern crate avm_lib;

use std::env;

fn main() {
	let args : Vec<String> = env::args().collect();
	let env = avm_lib::env::Env::new();
	env.run(&args);
}
