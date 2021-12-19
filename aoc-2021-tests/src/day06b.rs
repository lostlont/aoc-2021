#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day06::
	{
		Simulation,
		solution,
	};
	use aoc_2021_core::day06b::
	{
		solution_from,
	};
	use super::super::day06::tests::create_example_input;

	#[test]
	fn example_is_correct_for_256_days()
	{
		let simulation = Simulation::new(
			create_example_input(),
			8,
			6,
			256,
		);

		let actual = solution(simulation);

		assert_eq!(actual, 26984457539);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-06");
		let actual = solution_from(&path).unwrap();
		assert_eq!(actual, 1595779846729);
	}
}
