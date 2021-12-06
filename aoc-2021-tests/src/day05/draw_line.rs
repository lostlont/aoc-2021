#[cfg(test)]
mod test
{
	use aoc_2021_core::day05::
	{
		draw_line,
		Position,
		SparseTable,
	};

	#[test]
	fn draw_line_sets_values_according_to_line()
	{
		let mut table = SparseTable::new();
		let line = vec![
			Position::from(2, 3),
			Position::from(3, 3),
			Position::from(4, 3),
		];

		draw_line(&mut table, line);
		let mut actual = table
			.into_iter()
			.map(|(p, v)| (*p, *v))
			.collect::<Vec<_>>();
		actual.sort_by_key(|(p, _)| p.x());

		let expected = (2..=4)
			.map(|x| Position::from(x, 3))
			.map(|p| (p, 1))
			.collect::<Vec<_>>();
		assert_eq!(actual, expected);
	}

	#[test]
	fn draw_line_increments_already_existing_values()
	{
		let position = Position::from(1, 1);
		let mut table = SparseTable::new();
		table.set(position, 2);
		let line = vec![position];

		draw_line(&mut table, line);
		let actual = table.get(position);

		assert_eq!(actual, 3);
	}
}
