use std::path::Path;
use aoc_2021_core::solution_b_from;
use aoc_2021_core::solution_from;

fn main()
{
	let path = Path::new("input-01");
	let result = solution_from(&path);
	println!("{}", &result.to_string());

	let path = Path::new("input-01");
	let result = solution_b_from(&path);
	println!("{}", &result.to_string());
}
