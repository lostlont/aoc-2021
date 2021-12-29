use std::str::FromStr;
use thiserror::Error;
use super::Digit;
use super::ParseDigitError;

#[derive(Debug, PartialEq)]
pub struct Display
{
	signals: Vec<Digit>,
	output: Vec<Digit>,
}

impl Display
{
	pub fn new<TSignals, TOutput>(signals: TSignals, output: TOutput) -> Self
	where
		TSignals: IntoIterator<Item = Digit>,
		TOutput: IntoIterator<Item = Digit>,
	{
		Self
		{
			signals: signals.into_iter().collect(),
			output: output.into_iter().collect(),
		}
	}

	pub fn output(&self) -> impl Iterator<Item = &Digit>
	{
		self.output.iter()
	}
}

#[derive(Debug, Error)]
pub enum ParseDisplayError
{
	#[error("Could not split line by pattern ' | ': {0}!")]
	SplitError(String),

	#[error("Parse error: {0}!")]
	IntError(#[from] ParseDigitError),
}

impl FromStr for Display
{
	type Err = ParseDisplayError;

	fn from_str(string: &str) -> Result<Self, Self::Err>
	{
		match string.split(" | ")
			.collect::<Vec<_>>()
			.as_slice()
		{
			[signals, output] =>
			{
				let signal_digits = signals
					.split(' ')
					.map(|s| s.parse::<Digit>())
					.collect::<Result<Vec<_>, _>>()?;

				let output_digits = output
					.split(' ')
					.map(|s| s.parse::<Digit>())
					.collect::<Result<Vec<_>, _>>()?;

				Ok(Self::new(signal_digits, output_digits))
			},
			_ => Err(Self::Err::SplitError(string.to_string())),
		}
	}
}
