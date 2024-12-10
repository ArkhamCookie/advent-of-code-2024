use utils::{input_file, parse};

mod utils;

fn part_one(input: &str) -> i64 {
	let results: Vec<(&str, &str)> = parse(input);
	let mut output = 0;

	#[allow(clippy::needless_range_loop)] // Required for tuple
	for i in 0..results.len() {
		let (num1, num2) = results[i];
		let num1: i64 = num1.parse().unwrap();
		let num2: i64 = num2.parse().unwrap();

		output += num1 * num2;
	}

	output
}

fn main() {
	let file = input_file();

	let part_one_solution = part_one(&file);

	println!("Part 1: {}", part_one_solution);
}
