use thiserror::Error;
use super::Position;

#[derive(Debug, Error, PartialEq)]
pub enum LineError
{
	#[error("Arbitrary lines can not be handled: {start:?} -> {end:?}!")]
	ArbitraryLine
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
		let start_x = start.x() as i32;
		let start_y = start.y() as i32;
		let diff_x = end.x() as i32 - start_x;
		let diff_y = end.y() as i32 - start_y;
		if diff_x.abs() == diff_y.abs()
		{
			let sign_x = diff_x.signum();
			let sign_y = diff_y.signum();
			let length = diff_x.abs() + 1;
			let result = (0..length)
				.map(|v| Position::from(
					(start_x + v * sign_x) as usize,
					(start_y + v * sign_y) as usize))
				.collect();
			Ok(result)
		}
		else
		{
			Err(LineError::ArbitraryLine{ start, end })
		}
	}
}
