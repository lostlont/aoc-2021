#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day05a::
	{
		solution,
		solution_from,
	};
	use super::super::day05::tests::create_example_input;

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();
		let actual = solution(input);
		assert_eq!(actual, 5);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-05");
		let actual = solution_from(&path).unwrap();

		let expected = 5306;
		assert_eq!(actual, expected);
	}
}
