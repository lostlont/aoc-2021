#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day07b::
	{
		solution,
		solution_from,
	};
	use super::super::day07::tests::create_example_input;

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();

		let actual = solution(input)
			.unwrap();

		assert_eq!(actual, 168);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-07");

		let actual = solution_from(&path)
			.unwrap();

		assert_eq!(actual, 95476244);
	}
}
