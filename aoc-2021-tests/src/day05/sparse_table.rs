#[cfg(test)]
mod test
{
	use aoc_2021_core::day05::
	{
		Position,
		SparseTable,
	};

	#[test]
	fn get_returns_position_with_zero_for_nonexisting_entry()
	{
		let table = SparseTable::new();

		let actual = table.get(Position::from(100, 100));

		let expected = 0;
		assert_eq!(actual, expected);
	}

	#[test]
	fn set_changes_value_at_position()
	{
		let mut table = SparseTable::new();

		table.set(
			Position::from(100, 100),
			5);

		let actual = table.get(Position::from(100, 100));
		let expected = 5;
		assert_eq!(actual, expected);
	}

	#[test]
	fn into_iter_is_empty_for_no_entries()
	{
		let table = SparseTable::new();

		let actual = table.into_iter().collect::<Vec<_>>();

		let expected = vec![];
		assert_eq!(actual, expected);
	}

	#[test]
	fn into_iter_has_single_entry()
	{
		let mut table = SparseTable::new();
		table.set(
			Position::from(5, 4),
			3);

		let actual = table
			.into_iter()
			.map(|(k, v)| (*k, *v))
			.collect::<Vec<_>>();

		let expected = vec![(Position::from(5, 4), 3)];
		assert_eq!(actual, expected);
	}

	#[test]
	fn into_iter_has_all_entries_in_any_order()
	{
		let mut table = SparseTable::new();
		table.set(
			Position::from(10, 11),
			1);
		table.set(
			Position::from(12, 13),
			5);
		table.set(
			Position::from(10, 11),
			3);

		let actual = table
			.into_iter()
			.map(|(k, v)| (*k, *v))
			.collect::<Vec<_>>();

		let expected1 = vec![
			(Position::from(10, 11), 3),
			(Position::from(12, 13), 5),
		];
		let expected2 = vec![
			(Position::from(12, 13), 5),
			(Position::from(10, 11), 3),
		];
		assert!((actual == expected1) || (actual == expected2));
	}
}
