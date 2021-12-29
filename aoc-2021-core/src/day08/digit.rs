use std::collections::HashSet;
use std::str::FromStr;
use thiserror::Error;
use super::ParseSegmentError;
use super::Segment;

#[derive(Debug, PartialEq)]
pub struct Digit
{
	segments: HashSet<Segment>,
}

impl Digit
{
	pub fn new(segments: impl IntoIterator<Item = Segment>) -> Self
	{
		Self
		{
			segments: segments.into_iter().collect(),
		}
	}

	pub fn segments(&self) -> impl Iterator<Item = &Segment>
	{
		self.segments.iter()
	}
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseDigitError
{
	#[error("Parse error: {0}!")]
	ValueError(#[from] ParseSegmentError),
}

impl FromStr for Digit
{
	type Err = ParseDigitError;

	fn from_str(string: &str) -> Result<Self, Self::Err>
	{
		let segments = string
			.chars()
			.map(|c| c.to_string().parse::<Segment>())
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Digit::new(segments))
	}
}
