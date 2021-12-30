mod table;

#[cfg(test)]
pub mod tests
{
	use aoc_2021_core::common::Position;
	use aoc_2021_core::day09::
	{
		find_low_points,
		Table,
	};

	pub fn create_example_input() -> Table
	{
		Table::new([
			[2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
			[3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
			[9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
			[8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
			[9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
		])
		.unwrap()
	}

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
