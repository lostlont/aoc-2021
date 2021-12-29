use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Segment
{
	A,
	B,
	C,
	D,
	E,
	F,
	G,
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseSegmentError
{
	#[error("Parse error: {0}!")]
	ValueError(String),
}

impl FromStr for Segment
{
	type Err = ParseSegmentError;

	fn from_str(string: &str) -> Result<Self, Self::Err>
	{
		match string
		{
			"a" => Ok(Segment::A),
			"b" => Ok(Segment::B),
			"c" => Ok(Segment::C),
			"d" => Ok(Segment::D),
			"e" => Ok(Segment::E),
			"f" => Ok(Segment::F),
			"g" => Ok(Segment::G),
			_ => Err(ParseSegmentError::ValueError(string.to_string())),
		}
	}
}
