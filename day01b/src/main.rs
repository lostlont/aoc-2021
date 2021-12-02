use std::path::Path;
use day01b_core::solution_from;

fn main()
{
	let path = Path::new("../aoc-2021/input-01");
	let result = solution_from(&path);
	println!("{}", &result.to_string());
}
