#[cfg(test)]
mod test
{
	use aoc_2021_core::day05::Position;

	#[test]
	fn position_x()
	{
		let at = Position::from(2, 3);
		let actual = at.x();
		assert_eq!(actual, 2);
	}

	#[test]
	fn position_y()
	{
		let at = Position::from(2, 3);
		let actual = at.y();
		assert_eq!(actual, 3);
	}

	#[test]
	fn from_str_returns_error()
	{
		let actual = "".parse::<Position>();
		assert!(actual.is_err());
	}

	#[test]
	fn from_str_parses_zero()
	{
		let actual = "0,0".parse::<Position>();
		assert_eq!(actual, Ok(Position::from(0, 0)));
	}

	#[test]
	fn from_str_parses_values()
	{
		let actual = "1,2".parse::<Position>();
		assert_eq!(actual, Ok(Position::from(1, 2)));
	}
}
