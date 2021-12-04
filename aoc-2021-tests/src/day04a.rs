#[cfg(test)]
mod tests
{
	//use std::path::Path;
	use aoc_2021_core::day04a::
	{
		Game,
		Numbers,
		Table,
	};

	#[test]
	fn game_starts_unfinished()
	{
		let game = create_game();

		let actual = game.is_finished();

		let expected = false;
		assert_eq!(actual, expected);
	}

	#[test]
	fn game_step_pulls_number()
	{
		let mut game = create_game();

		game.step();
		let number_count = game.numbers().count();

		assert_eq!(number_count, 3);
	}

	#[test]
	fn game_step_marks_pulled_number_on_tables()
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
	fn game_step_leaves_game_unfinished_if_no_table_has_bingo()
	{
		let mut game = create_game();

		game.step();
		let actual = game.is_finished();

		assert_eq!(actual, false);
	}

	#[test]
	fn game_step_set_game_finished_if_a_table_has_bingo()
	{
		let mut game = create_game();

		game.step();
		game.step();
		let actual = game.is_finished();

		assert_eq!(actual, true);
	}

	#[test]
	fn game_score_is_correct()
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

	#[test]
	fn example_is_correct()
	{
		let mut game = create_example_game();

		while !game.is_finished()
		{
			game.step();
		}
		let actual = game.score();

		assert_eq!(actual, 4512);
	}

	fn create_example_game() -> Game
	{
		Game::from(
			Numbers::from(vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1]),
			vec![
				Table::from(vec![
					vec![22, 13, 17, 11, 0],
					vec![8, 2, 23, 4, 24],
					vec![21, 9, 14, 16, 7],
					vec![6, 10, 3, 18, 5],
					vec![1, 12, 20, 15, 19],
				]),
				Table::from(vec![
					vec![3, 15, 0, 2, 22],
					vec![9, 18, 13, 17, 5],
					vec![19, 8, 7, 25, 23],
					vec![20, 11, 10, 24, 4],
					vec![14, 21, 16, 12, 6],
				]),
				Table::from(vec![
					vec![14, 21, 17, 24, 4],
					vec![10, 16, 15, 9, 19],
					vec![18, 8, 23, 26, 20],
					vec![22, 11, 13, 6, 5],
					vec![2, 0, 12, 3, 7],
				]),
			])
	}
}
