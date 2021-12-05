mod game;
mod numbers;
mod table;

#[cfg(test)]
pub mod test
{
	use aoc_2021_core::day04::
	{
		Game,
		Numbers,
		Table,
	};

	pub fn create_example_game() -> Game
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
