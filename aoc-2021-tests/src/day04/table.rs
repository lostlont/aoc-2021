#[cfg(test)]
mod tests
{
	use aoc_2021_core::day04::Table;

	#[test]
	fn positions_iterates_all()
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
	fn number_at_returns_correct_number()
	{
		let numbers = vec![
			vec![1, 2],
			vec![3, 4],
		];
		let table = Table::from(numbers.clone());

		let actual = table
			.positions()
			.all(|p| table.number_at(p.0, p.1) == numbers[p.1][p.0]);

		let expected = true;
		assert_eq!(actual, expected);
	}

	#[test]
	fn starts_unmarked()
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
	fn mark_marks_position()
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
	fn is_bingo_returns_false_when_no_bingo()
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
	fn is_bingo_returns_true_when_a_full_row_is_marked()
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
	fn is_bingo_returns_true_when_a_full_column_is_marked()
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
}
