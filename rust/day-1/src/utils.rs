use std::env;
use std::fs;
use std::process::exit;

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

pub(crate) fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
	let mut left_column: Vec<i64> = Vec::new();
	let mut right_column: Vec<i64> = Vec::new();

	for line in input.lines() {
		if line.is_empty() {
			continue;
		}

		let numbers: Vec<&str> = line.split_whitespace().collect();
		let left_number: i64 = numbers[0].parse().unwrap();
		let right_number: i64 = numbers[1].parse().unwrap();

		left_column.push(left_number);
		right_column.push(right_number);
	};

	(left_column, right_column)
}
