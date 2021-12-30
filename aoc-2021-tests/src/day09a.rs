#[cfg(test)]
mod tests
{
	use aoc_2021_core::common::Position;
	use aoc_2021_core::day09::Table;
	use aoc_2021_core::day09a::find_low_points;
	use super::super::day09::tests::create_example_input;

	#[test]
	fn find_low_points_is_correct()
	{
		let input = [[1, 2], [3, 4]];
		let table = Table::new(input)
			.unwrap();

		let actual = find_low_points(&table)
			.into_iter()
			.collect::<Vec<_>>();

		let at = Position::from(0, 0);
		let expected = vec![(at, 1)];
		assert_eq!(actual, expected);
	}

	#[test]
	fn find_low_points_is_correct_for_example()
	{
		let table = create_example_input();

		let actual = find_low_points(&table)
			.into_iter()
			.collect::<Vec<_>>();

		let expected = vec![
			(Position::from(1, 0), 1),
			(Position::from(9, 0), 0),
			(Position::from(2, 2), 5),
			(Position::from(6, 4), 5),
		];
		assert_eq!(actual, expected);
	}
}
