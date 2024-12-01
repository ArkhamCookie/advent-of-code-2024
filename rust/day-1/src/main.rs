use std::env;
use std::fs;
use std::process::exit;

fn main() {
	let path = match env::args().nth(1) {
		Some(arg) => arg,
		None => {
			println!("Missing required arguement: <input path>");
			exit(1);
		}
	};
	let file = match fs::read_to_string(path) {
		Ok(file) => file,
		Err(error) => {
			println!("[ERROR] Issue reading file: {}", error);
			exit(1);
		}
	};

	println!("{}", file);
}
