#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day09::Table;
	use aoc_2021_core::day09a::
	{
		solve,
		solve_from,
	};
	use super::super::day09::tests::create_example_input;

	#[test]
	fn example_is_correct()
	{
		let table = create_example_input();
		let actual = solve(&table);
		assert_eq!(actual, 15);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-09");

		let actual = solve_from(&path)
			.unwrap();

		assert_eq!(actual, 494);
	}
}
