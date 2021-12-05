#[cfg(test)]
mod test
{
	use aoc_2021_core::day05::Position;

	#[test]
	fn position_x()
	{
		let at = Position::from(2, 3);

		let actual = at.x();

		let expected = 2;
		assert_eq!(actual, expected);
	}

	#[test]
	fn position_y()
	{
		let at = Position::from(2, 3);

		let actual = at.y();

		let expected = 3;
		assert_eq!(actual, expected);
	}
}
