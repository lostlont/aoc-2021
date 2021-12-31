#[cfg(test)]
mod tests
{
	//use std::path::Path;
	use aoc_2021_core::day10b::
	{
		solve,
		//solve_from,
	};
	use super::super::day10::tests::create_example_input;

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();

		let actual = solve(input.as_bytes())
			.unwrap();

		assert_eq!(actual, 288957);
	}
}
