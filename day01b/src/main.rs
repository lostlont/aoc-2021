use std::path::Path;
use day01b_core::solution_from;

fn main()
{
	let path = Path::new("../day01/input");
	let result = solution_from(&path);
	println!("{}", &result.to_string());
}
