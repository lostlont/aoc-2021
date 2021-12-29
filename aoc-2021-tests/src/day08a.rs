#[cfg(test)]
mod tests
{
	use std::path::Path;
	use super::super::day08::tests::create_example_input;
	use aoc_2021_core::day08::Board;
	use aoc_2021_core::day08a::
	{
		solution,
		solution_from,
	};

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();
		let board = Board::parse(input.as_bytes()).unwrap();

		let actual = solution(&board);

		assert_eq!(actual, 26);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-08");

		let actual = solution_from(&path)
			.unwrap();

		assert_eq!(actual, 440);
	}
}
