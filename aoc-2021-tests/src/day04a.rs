#[cfg(test)]
mod tests
{
	//use std::path::Path;
	use aoc_2021_core::day04a::
	{
		Numbers,
		Table,
	};

	#[test]
	fn numbers_pull_returns_only()
	{
		let mut numbers = Numbers::from(vec![3]);

		let actual = numbers.pull();

		let expected = 3;
		assert_eq!(actual, expected);
	}

	#[test]
	fn numbers_pull_decreases_count()
	{
		let mut numbers = Numbers::from(vec![3]);
		numbers.pull();

		let actual = numbers.count();

		let expected = 0;
		assert_eq!(actual, expected);
	}

	#[test]
	fn numbers_pull_pops_first()
	{
		let mut numbers = Numbers::from(vec![3, 5]);
		numbers.pull();

		let actual = numbers.pull();

		let expected = 5;
		assert_eq!(actual, expected);
	}

	#[test]
	fn table_positions_iterates_all()
	{
		let table = Table::from(vec![
			vec![1, 2],
			vec![3, 4],
		]);

		let actual = table.positions().collect::<Vec<_>>();

		let expected = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
		assert_eq!(actual, expected);
	}

	#[test]
	fn table_starts_unmarked()
	{
		let table = Table::from(vec![
			vec![1, 2],
			vec![3, 4],
		]);

		let actual = table
			.positions()
			.all(|p| table.is_marked_at(p.0, p.1) == false);

		let expected = true;
		assert_eq!(actual, expected);
	}

	#[test]
	fn table_mark_marks_position()
	{
		let mut table = Table::from(vec![
			vec![1, 2],
			vec![3, 4],
		]);

		table.mark(1, 1);

		let actual = table
			.positions()
			.all(|p|
				{
					let should_be_marked = (p.0 == 1) && (p.1 == 1);
					table.is_marked_at(p.0, p.1) == should_be_marked
				});

		let expected = true;
		assert_eq!(actual, expected);
	}

	#[test]
	fn table_is_bingo_returns_false_when_no_bingo()
	{
		let table = Table::from_marks(
			vec![
				vec![1, 2],
				vec![3, 4],
			],
			vec![
				vec![false, true],
				vec![true, false],
			]);

		let actual = table.is_bingo();

		let expected = false;
		assert_eq!(actual, expected);
	}

	#[test]
	fn table_is_bingo_returns_true_when_a_full_row_is_marked()
	{
		let table = Table::from_marks(
			vec![
				vec![1, 2],
				vec![3, 4],
			],
			vec![
				vec![false, false],
				vec![true, true],
			]);

		let actual = table.is_bingo();

		let expected = true;
		assert_eq!(actual, expected);
	}

	#[test]
	fn table_is_bingo_returns_true_when_a_full_column_is_marked()
	{
		let table = Table::from_marks(
			vec![
				vec![1, 2],
				vec![3, 4],
			],
			vec![
				vec![false, true],
				vec![false, true],
			]);

		let actual = table.is_bingo();

		let expected = true;
		assert_eq!(actual, expected);
	}

	fn create_example_table1() -> Table
	{
		Table::from(vec![
			vec![22, 13, 17, 11, 0],
			vec![8, 2, 23, 4, 24],
			vec![21, 9, 14, 16, 7],
			vec![6, 10, 3, 18, 5],
			vec![1, 12, 20, 15, 19],
		])
	}
}
