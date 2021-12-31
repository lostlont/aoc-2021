use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;
use super::day10::character_points;
use super::day10::check_line;
use super::day10::LineStatus;

#[derive(Debug, Error)]
pub enum ParseLineError
{
	#[error("IO error: {0}!")]
	IoError(#[from] std::io::Error),
}

pub fn solve_from(path: &Path) -> Result<i32, ParseLineError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	solve(reader)
}

pub fn solve(input: impl BufRead) -> Result<i32, ParseLineError>
{
	let mut result = 0;

	for line in input.lines()
	{
		let line = line?;
		let status = check_line(&line);
		if let LineStatus::Corrupted(ch) = status
		{
			result += character_points(ch);
		}
	}

	Ok(result)
}
