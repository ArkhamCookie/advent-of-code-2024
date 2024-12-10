use utils::{difference, parse};

use shared::input_file;

mod utils;

fn part_one(input: &str) -> (i64, Vec<i64>) {
	let (column_one, column_two, column_three, column_four, column_five) = parse(input);
	let mut safe_amount = 0;
	let mut safe_reports: Vec<i64> = Vec::new();

	for i in 0..column_one.len() {
		let mut difference_amount: i64;

		// Comparing 1st to 2nd
		difference_amount = difference(column_one[i], column_two[i]);

		if difference_amount > 3 && difference_amount != 0 {
			continue;
		}

		// Comparing 2nd to 3rd
		difference_amount = difference(column_two[i], column_three[i]);

		if difference_amount > 3 && difference_amount != 0 {
			continue;
		}

		// Comparing 3rd to 4th
		difference_amount = difference(column_three[i], column_four[i]);

		if difference_amount > 3 && difference_amount != 0 {
			continue;
		}

		// Comparing 4th to 5th
		difference_amount = difference(column_four[i], column_five[i]);

		if difference_amount > 3 && difference_amount != 0 {
			continue;
		}

		safe_reports.push((i + 1).try_into().unwrap());
		safe_amount += 1;
	}

	(safe_amount, safe_reports)
}

fn main() {
	let file = input_file();

	let (part_one_solution, _safe_reports) = part_one(&file);

	println!("Part 1: {}", part_one_solution);
	#[cfg(debug_assertions)]
	println!("Safe Reports: {:?}", _safe_reports);
}
