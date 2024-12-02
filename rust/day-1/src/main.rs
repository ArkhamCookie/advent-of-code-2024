use crate::utils::{input_file, parse};

mod utils;

fn main() {
	let file = input_file();

	let solution = part_one(&file);

	println!("{}", solution);
}

fn part_one(input: &str) -> i64 {
	let (mut left_column, mut right_column) = parse(input);

	left_column.sort();
	right_column.sort();

	let mut sum = 0;

	for i in 0..left_column.len() {
		if left_column[i] > right_column[i] {
			sum += left_column[i] - right_column[i];
		} else {
			sum += right_column[i] - left_column[i];
		}
	}

	sum
}
