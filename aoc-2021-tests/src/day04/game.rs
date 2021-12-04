#[cfg(test)]
mod tests
{
	use aoc_2021_core::day04a::
	{
		Game,
		Numbers,
		Table,
	};

	#[test]
	fn starts_unfinished()
	{
		let game = create_game();

		let actual = game.is_finished();

		let expected = false;
		assert_eq!(actual, expected);
	}

	#[test]
	fn step_pulls_number()
	{
		let mut game = create_game();

		game.step();
		let number_count = game.numbers().count();

		assert_eq!(number_count, 3);
	}

	#[test]
	fn step_marks_pulled_number_on_tables()
	{
		let mut game = create_game();

		game.step();
		let actual = game
			.tables()
			.iter()
			.all(|table| table
				.positions()
				.all(|p|
					{
						let should_be_marked = table.number_at(p.0, p.1) == 3;
						let is_marked = table.is_marked_at(p.0, p.1);
						is_marked == should_be_marked
					}));

		assert_eq!(actual, true);
	}

	#[test]
	fn step_leaves_game_unfinished_if_no_table_has_bingo()
	{
		let mut game = create_game();

		game.step();
		let actual = game.is_finished();

		assert_eq!(actual, false);
	}

	#[test]
	fn step_set_game_finished_if_a_table_has_bingo()
	{
		let mut game = create_game();

		game.step();
		game.step();
		let actual = game.is_finished();

		assert_eq!(actual, true);
	}

	#[test]
	fn score_is_correct()
	{
		let mut game = create_game();

		game.step();
		game.step();
		let actual = game.score();

		assert_eq!(actual, (4 + 2) * 1);
	}

	fn create_game() -> Game
	{
		let numbers = Numbers::from(vec![3, 1, 2, 4]);
		let tables = vec![
			Table::from(vec![
				vec![1, 2],
				vec![4, 3],
			]),
			Table::from(vec![
				vec![3, 4],
				vec![1, 2],
			]),
		];
		Game::from(numbers, tables)

	}
}
