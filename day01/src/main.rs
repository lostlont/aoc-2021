use std::path::Path;
use day01_core::solution;

fn main()
{
	let path = Path::new("input");
	let result = solution(&path);
	println!("{}", &result.to_string());
}
