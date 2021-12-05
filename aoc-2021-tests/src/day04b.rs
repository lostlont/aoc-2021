#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day04b::
	{
		solution,
		solution_from,
	};
	use crate::day04::test::create_example_game;

	#[test]
	fn example_is_correct()
	{
		let game = create_example_game();

		let actual = solution(game);

		assert_eq!(actual, 1924);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-04");
		let actual = solution_from(&path);

		let expected = 7075;
		assert_eq!(actual, expected);
	}
}
