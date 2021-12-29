use std::io::BufRead;
use thiserror::Error;
use super::Display;
use super::ParseDisplayError;

pub struct Board
{
	displays: Vec<Display>,
}

#[derive(Debug, Error)]
pub enum ParseBoardError
{
	#[error("IO error: {0}!")]
	IoError(#[from] std::io::Error),

	#[error("Parse error: {0}!")]
	ValueError(#[from] ParseDisplayError),
}

impl Board
{
	pub fn parse(input: impl BufRead) -> Result<Self, ParseBoardError>
	{
		let displays = input
			.lines()
			.collect::<Result<Vec<_>, _>>()?
			.iter()
			.map(|l| l.parse::<Display>())
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Self::new(displays))
	}

	pub fn new(displays: impl IntoIterator<Item = Display>) -> Self
	{
		Self
		{
			displays: displays.into_iter().collect(),
		}
	}

	pub fn displays(&self) -> impl Iterator<Item = &Display>
	{
		self.displays.iter()
	}
}
