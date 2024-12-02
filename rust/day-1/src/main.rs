use crate::utils::{input_file, parse};

mod utils;

fn main() {
	let file = input_file();

	let part_one_solution = part_one(&file);
	let part_two_solution = part_two(&file);

	println!("Part 1: {}", part_one_solution);
	println!("Part 2: {}", part_two_solution);
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

fn part_two(input: &str) -> i64 {
	let (left_column, right_column) = parse(input);
	let mut similarity = 0;

	for left_number in left_column {
		let mut count = 0;

		for right_number in right_column.clone() {
			if right_number == left_number {
				count += 1;
			}
		}
		similarity += left_number * count;
	}

	similarity
}
