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

#[allow(clippy::type_complexity)]
pub(crate) fn parse(input: &str) -> (Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>) {
	let mut column_one: Vec<i64> = Vec::new();
	let mut column_two: Vec<i64> = Vec::new();
	let mut column_three: Vec<i64> = Vec::new();
	let mut column_four: Vec<i64> = Vec::new();
	let mut column_five: Vec<i64> = Vec::new();

	for line in input.lines() {
		if line.is_empty() {
			continue;
		}

		let numbers: Vec<&str> = line.split_whitespace().collect();
		let first_number: i64 = numbers[0].parse().unwrap();
		let second_number: i64 = numbers[1].parse().unwrap();
		let third_number: i64 = numbers[2].parse().unwrap();
		let fourth_number: i64 = numbers[3].parse().unwrap();
		let fifth_number: i64 = numbers[4].parse().unwrap();

		column_one.push(first_number);
		column_two.push(second_number);
		column_three.push(third_number);
		column_four.push(fourth_number);
		column_five.push(fifth_number);
	}

	(column_one, column_two, column_three, column_four, column_five)
}

pub(crate) fn difference(number_one: i64, number_two: i64) -> i64 {
	if number_one > number_two {
		number_one - number_two
	} else {
		number_two - number_one
	}
}
