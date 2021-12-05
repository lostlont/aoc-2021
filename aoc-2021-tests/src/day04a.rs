#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day04a::
	{
		Game,
		Numbers,
		solution,
		solution_from,
		Table,
	};

	#[test]
	fn example_is_correct()
	{
		let game = create_example_game();

		let actual = solution(game);

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

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-04");
		let actual = solution_from(&path);

		let expected = 16674;
		assert_eq!(actual, expected);
	}
}
