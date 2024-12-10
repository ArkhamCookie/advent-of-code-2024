use utils::{input_file, parse, sum_mul};

mod utils;

fn part_one(input: &str) -> i64 {
	let parsed = parse(input);

	sum_mul(parsed)
}

fn main() {
	let file = input_file();

	let part_one_solution = part_one(&file);

	println!("Part 1: {}", part_one_solution);
}
