use std::io::prelude::*;
use thiserror::Error;
use super::day05::ParsePositionError;
use super::day05::Position;

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
