use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::day07::median;
use super::day07::parse;
use super::day07::ParseError;

pub fn solution(input: Vec<i32>) -> Option<i32>
{
	if let Some(median) = median(input.clone())
	{
		input
			.iter()
			.map(|v| (v - median).abs())
			.sum::<i32>()
			.into()
	}
	else
	{
		None
	}
}

pub fn solution_from(path: &Path) -> Result<i32, ParseError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let input = parse(reader)?;

	solution(input)
		.ok_or(ParseError::EmptyFileError)
}
