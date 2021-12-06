use thiserror::Error;
use super::Position;

#[derive(Debug, Error, PartialEq)]
pub enum LineError
{
	#[error("Diagonal lines can not be handled: {start:?} -> {end:?}!")]
	DiagonalLine
	{
		start: Position,
		end: Position,
	},
}

pub fn line(start: Position, end: Position) -> Result<Vec<Position>, LineError>
{
	if start == end
	{
		Ok(vec![start])
	}
	else if start.x() == end.x()
	{
		let result = if start.y() < end.y()
		{
			(start.y()..=end.y())
				.map(|y| Position::from(start.x(), y))
				.collect()
		}
		else
		{
			(end.y()..=start.y())
				.rev()
				.map(|y| Position::from(start.x(), y))
				.collect()
		};
		Ok(result)
	}
	else if start.y() == end.y()
	{
		let result = if start.x() < end.x()
		{
			(start.x()..=end.x())
				.map(|x| Position::from(x, start.y()))
				.collect()
		}
		else
		{
			(end.x()..=start.x())
				.rev()
				.map(|x| Position::from(x, start.y()))
				.collect()
		};
		Ok(result)
	}
	else
	{
		Err(LineError::DiagonalLine{ start, end })
	}
}
