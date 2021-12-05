use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position
{
	x: usize,
	y: usize,
}

impl Position
{
	pub fn from(x: usize, y: usize) -> Self
	{
		Self
		{
			x,
			y,
		}
	}

	pub fn x(&self) -> usize
	{
		self.x
	}

	pub fn y(&self) -> usize
	{
		self.y
	}
}

#[derive(Debug, Error, PartialEq)]
pub enum ParsePositionError
{
	#[error("String does not contain two values: {0:?}!")]
	SplitError(String),

	#[error("Parse error: {0}!")]
	ValueError(#[from] ParseIntError),
}

impl FromStr for Position
{
	type Err = ParsePositionError;

	fn from_str(string: &str) -> Result<Self, Self::Err>
	{
		let mut split = string.split(',');
		let x = split
			.next()
			.ok_or(ParsePositionError::SplitError(string.to_string()))?
			.parse::<usize>()?;
		let y = split
			.next()
			.ok_or(ParsePositionError::SplitError(string.to_string()))?
			.parse::<usize>()?;

		Ok(Position::from(x, y))
	}
}
