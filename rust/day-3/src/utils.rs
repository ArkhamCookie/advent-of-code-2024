use regex::Regex;

pub(crate) fn parse(input: &str) -> Vec<(&str, &str)> {
	let regex = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
	let results: Vec<(&str, &str)> = regex.captures_iter(input).map(|captures| {
		let num1 = captures.name("num1").unwrap().as_str();
		let num2 = captures.name("num2").unwrap().as_str();

		(num1, num2)
	}).collect();

	results
}

pub(crate) fn sum_mul(input: Vec<(&str, &str)>) -> i64 {
	let mut output = 0;

	#[allow(clippy::needless_range_loop)] // Required for tuple
	for i in 0..input.len() {
		let (num1, num2) = input[i];
		let num1: i64 = num1.parse().unwrap();
		let num2: i64 = num2.parse().unwrap();

		output += num1 * num2;
	}

	output
}
