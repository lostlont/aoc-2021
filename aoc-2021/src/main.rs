use std::path::Path;
use aoc_2021_core::
{
	day01a,
	day01b,
};

fn main()
{
	let path = Path::new("input-01");
	let result = day01a::solution_from(&path);
	println!("{}", &result.to_string());

	let path = Path::new("input-01");
	let result = day01b::solution_from(&path);
	println!("{}", &result.to_string());
}
