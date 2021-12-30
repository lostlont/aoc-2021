#[cfg(test)]
mod tests
{
	use std::collections::HashSet;
	use aoc_2021_core::common::Position;
	use aoc_2021_core::day09::Table;
	use aoc_2021_core::day09b::
	{
		find_basin,
		solve,
	};
	use super::super::day09::tests::create_example_input;

	#[test]
	fn find_basin_returns_all_not_9_neighbor_positions()
	{
		let input = [
			[4, 5, 9],
			[2, 3, 5],
		];
		let table = Table::new(input)
			.unwrap();
		let at = Position::from(0, 1);

		let actual = find_basin(&table, at)
			.into_iter()
			.collect::<Vec<_>>();
		let actual : HashSet<Position> = HashSet::from_iter(actual);

		let expected = vec![
			Position::from(0, 0),
			Position::from(1, 0),
			Position::from(0, 1),
			Position::from(1, 1),
			Position::from(2, 1),
		];
		let expected = HashSet::from_iter(expected);
		assert_eq!(actual, expected);
	}

	#[test]
	fn example_is_correct()
	{
		let table = create_example_input();
		let actual = solve(&table);
		assert_eq!(actual, 1134);
	}
}
