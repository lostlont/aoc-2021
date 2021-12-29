#[cfg(test)]
mod tests
{
	use aoc_2021_core::day07a::
	{
		solution,
	};
	use super::super::day07::tests::create_example_input;

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();

		let actual = solution(input);

		let expected = Some(37);
		assert_eq!(actual, expected);
	}
}
