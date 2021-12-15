use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;
use super::day05::draw_line;
use super::day05::line;
use super::day05::ParsePositionError;
use super::day05::Position;
use super::day05::SparseTable;

#[derive(Debug, Error)]
pub enum ParseLineError
{
	#[error("Parse error: {0}!")]
	IoError(#[from] std::io::Error),

	#[error("Could not split line by pattern ' -> ': {0}!")]
	SplitError(String),

	#[error("Position error: {0}!")]
	ParsePositionError(#[from] ParsePositionError),
}

pub fn solution(input: impl IntoIterator<Item = (Position, Position)>) -> i32
{
	let mut table = SparseTable::new();
	let positions = input
		.into_iter()
		.filter_map(|(a, b)| line(a, b).ok())
		.flatten()
		.collect::<Vec<_>>();

	draw_line(&mut table, positions);
	table
		.into_iter()
		.map(|(_, v)| *v)
		.filter(|v| *v >= 2)
		.count() as i32
}

pub fn parse(input: impl BufRead) -> Result<Vec<(Position, Position)>, ParseLineError>
{
	input
		.lines()
		.collect::<Result<Vec<_>, _>>()?
		.iter()
		.filter(|line| !line.is_empty())
		.map(|line|
			match line
				.split(" -> ")
				.collect::<Vec<_>>()
				.as_slice()
			{
				[a, b] => match (a.parse::<Position>(), b.parse::<Position>())
				{
					(Ok(a), Ok(b)) => Ok((a, b)),
					(Err(a), _) => Err(ParseLineError::ParsePositionError(a)),
					(_, Err(b)) => Err(ParseLineError::ParsePositionError(b)),
				},
				_ => Err(ParseLineError::SplitError(line.clone())),
			})
		.collect()
}

pub fn solution_from(path: &Path) -> Result<i32, ParseLineError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let input = parse(reader)?;
	Ok(solution(input))
}
