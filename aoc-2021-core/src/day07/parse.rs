use std::io::BufRead;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError
{
	#[error("IO error: {0}!")]
	IoError(#[from] std::io::Error),

	#[error("File was empty!")]
	EmptyFileError,

	#[error("Int error: {0}!")]
	IntError(#[from] ParseIntError),
}

pub fn parse(input: impl BufRead) -> Result<Vec<i32>, ParseError>
{
	Ok(input
		.lines()
		.next()
		.ok_or(ParseError::EmptyFileError)??
		.split(',')
		.map(|v| v.parse::<i32>())
		.collect::<Result<Vec<_>, _>>()?)
}
