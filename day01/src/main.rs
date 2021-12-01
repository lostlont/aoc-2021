use std::path::Path;
use day01_core::solution_from;

fn main()
{
	let path = Path::new("input");
	let result = solution_from(&path);
	println!("{}", &result.to_string());
}
