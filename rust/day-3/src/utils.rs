use std::env;
use std::fs;
use std::process::exit;

use regex::Regex;

pub(crate) fn input_file() -> String {
	let path = match env::args().nth(1) {
		Some(arg) => arg,
		None => {
			println!("Missing required arguement: <input path>");
			exit(1);
		}
	};

	match fs::read_to_string(path) {
		Ok(file) => file,
		Err(error) => {
			println!("[ERROR] Issue reading file: {}", error);
			exit(1);
		}
	}
}

pub(crate) fn parse(input: &str) -> Vec<(&str, &str)> {
	let regex = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
	let results: Vec<(&str, &str)> = regex.captures_iter(input).map(|captures| {
		let num1 = captures.name("num1").unwrap().as_str();
		let num2 = captures.name("num2").unwrap().as_str();

		(num1, num2)
	}).collect();

	results
}
