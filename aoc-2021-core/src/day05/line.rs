use std::cmp::max;
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
	let start_x = start.x() as i32;
	let start_y = start.y() as i32;
	let diff_x = end.x() as i32 - start_x;
	let diff_y = end.y() as i32 - start_y;
	let sign_x = diff_x.signum();
	let sign_y = diff_y.signum();
	let length = max(
		diff_x.abs(),
		diff_y.abs())
		+ 1;

	let valid =
		(diff_x == 0) ||
		(diff_y == 0) ||
		(diff_x.abs() == diff_y.abs());
	if valid
	{
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
