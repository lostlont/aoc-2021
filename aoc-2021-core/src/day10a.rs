use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use super::day10::check_line;
use super::day10::LineStatus;
use super::day10::ParseLineError;

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

fn character_points(ch: char) -> i32
{
	match ch
	{
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => 0,
	}
}
