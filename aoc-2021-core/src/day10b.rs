//use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::path::Path;
//use thiserror::Error;
use super::common::math;
use super::day10::check_line;
use super::day10::LineStatus;
use super::day10::ParseLineError;

pub fn solve(input: impl BufRead) -> Result<i32, ParseLineError>
{
	let mut points = vec![];

	for line in input.lines()
	{
		let line = line?;
		let status = check_line(&line);
		if let LineStatus::Incomplete(stack) = status
		{
			let point = completion_points(&stack);
			points.push(point);
		}
	}

	let result = math::median(points)
		.ok_or(ParseLineError::EmptyLineError)?;

	Ok(result)
}

fn completion_points(stack: &Vec<char>) -> i32
{
	stack
		.iter()
		.cloned()
		.rev()
		.map(character_points)
		.fold(0, |acc, v| acc * 5 + v)
}

fn character_points(ch: char) -> i32
{
	match ch
	{
		'(' => 1,
		'[' => 2,
		'{' => 3,
		'<' => 4,
		_ => 0,
	}
}
