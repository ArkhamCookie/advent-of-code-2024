use utils::{input_file, parse, sum_mul};

mod utils;

fn part_one(input: &str) -> i64 {
	let parsed = parse(input);

	sum_mul(parsed)
}

fn part_two(input: &str) -> i64 {
	let mut sum = 0;

	for i in input.split("do()") {
		let do_code = i.split("don't()").next().unwrap_or("");
		let parsed = parse(do_code);
		sum += sum_mul(parsed);
	}

	sum
}

fn main() {
	let file = input_file();

	let part_one_solution = part_one(&file);
	let part_two_solution = part_two(&file);

	println!("Part 1: {}", part_one_solution);
	println!("Part 2: {}", part_two_solution);
}
