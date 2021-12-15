#[cfg(test)]
mod test
{
	use aoc_2021_core::day05::
	{
		line,
		LineError,
		Position,
	};

	#[test]
	fn line_has_single_position_for_same_start_end()
	{
		let start = Position::from(2, 3);
		let end = Position::from(2, 3);

		let actual = line(start, end);

		let expected = Ok(vec![start]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_has_two_positions_for_end_right_to_start()
	{
		let start = Position::from(5, 5);
		let end = Position::from(6, 5);

		let actual = line(start, end);

		let expected = Ok(vec![start, end]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_has_two_positions_for_end_left_to_start()
	{
		let start = Position::from(5, 5);
		let end = Position::from(4, 5);

		let actual = line(start, end);

		let expected = Ok(vec![start, end]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_has_two_positions_for_end_below_start()
	{
		let start = Position::from(5, 5);
		let end = Position::from(5, 6);

		let actual = line(start, end);

		let expected = Ok(vec![start, end]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_has_two_positions_for_end_above_start()
	{
		let start = Position::from(5, 5);
		let end = Position::from(5, 4);

		let actual = line(start, end);

		let expected = Ok(vec![start, end]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_has_multiple_positions_for_long_line()
	{
		let start = Position::from(5, 5);
		let end = Position::from(10, 5);

		let actual = line(start, end);

		let expected = (5..=10)
			.map(|x| Position::from(x, 5))
			.collect::<Vec<_>>();
		let expected = Ok(expected);
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_handles_diagonal_line()
	{
		let start = Position::from(5, 5);
		let end = Position::from(6, 6);

		let actual = line(start, end)
			.unwrap();

		let expected = vec![start, end];
		assert_eq!(actual, expected);
	}

	#[test]
	fn line_returns_error_for_arbitrary_line()
	{
		let start = Position::from(0, 0);
		let end = Position::from(2, 1);

		let actual = line(start, end);

		let expected = Err(LineError::ArbitraryLine{ start, end });
		assert_eq!(actual, expected);
	}
}
