#[cfg(test)]
mod tests
{
	use aoc_2021_core::common::Position;
	use aoc_2021_core::day09::Table;

	#[test]
	fn new_returns_none_for_zero_rows()
	{
		let input : [[i32; 3]; 0] = [];
		let actual = Table::new(input);
		assert_eq!(actual, None);
	}

	#[test]
	fn new_returns_none_for_zero_columns()
	{
		let input : [[i32; 0]; 3] = [[], [], []];
		let actual = Table::new(input);
		assert_eq!(actual, None);
	}

	#[test]
	fn new_returns_table_for_nonempty_input()
	{
		let input = [[1, 2], [3, 4]];
		let actual = Table::new(input);
		assert!(actual.is_some());
	}

	#[test]
	fn new_returns_none_for_nonuniform_row_count()
	{
		let input = vec![
			vec![1, 2],
			vec![3, 4, 5],
		];

		let actual = Table::new(input);

		assert_eq!(actual, None);
	}

	#[test]
	fn get_returns_none_for_invalid_position()
	{
		let table = Table::new([[5]])
			.unwrap();

		let at = Position::from(10, 10);
		let actual = table.get(at);

		assert_eq!(actual, None);
	}

	#[test]
	fn get_returns_value_for_valid_position()
	{
		let input = [[1, 2], [3, 4]];
		let table = Table::new(input)
			.unwrap();

		let at = Position::from(1, 0);
		let actual = table.get(at);

		assert_eq!(actual, Some(2));
	}

	#[test]
	fn set_changes_value_for_valid_position()
	{
		let input = [[1, 2], [3, 4]];
		let mut table = Table::new(input)
			.unwrap();

		let at = Position::from(1, 0);
		table.set(at, 5);

		let input = [[1, 5], [3, 4]];
		let expected = Table::new(input)
			.unwrap();
		assert_eq!(table, expected);
	}

	#[test]
	fn set_does_not_change_value_for_invalid_position()
	{
		let input = [[1, 2], [3, 4]];
		let mut table = Table::new(input)
			.unwrap();

		let at = Position::from(5, 5);
		table.set(at, 5);

		let expected = Table::new(input)
			.unwrap();
		assert_eq!(table, expected);
	}

	#[test]
	fn into_iter_returns_all_entries_in_row_major_order()
	{
		let input = [[1, 2], [3, 4]];
		let table = Table::new(input)
			.unwrap();

		let actual = table
			.into_iter()
			.collect::<Vec<_>>();

		let expected = vec![
			(Position::from(0, 0), 1),
			(Position::from(1, 0), 2),
			(Position::from(0, 1), 3),
			(Position::from(1, 1), 4),
		];
		assert_eq!(actual, expected);
	}
}
