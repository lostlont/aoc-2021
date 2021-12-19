#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day06::
	{
		Simulation,
		solution,
	};
	use aoc_2021_core::day06a::
	{
		solution_from,
	};
	use super::super::day06::tests::create_example_input;

	#[test]
	fn example_is_correct_for_18_days()
	{
		let simulation = Simulation::new(
			create_example_input(),
			8,
			6,
			18,
		);

		let actual = solution(simulation);

		assert_eq!(actual, 26);
	}

	#[test]
	fn example_is_correct_for_80_days()
	{
		let simulation = Simulation::new(
			create_example_input(),
			8,
			6,
			80,
		);

		let actual = solution(simulation);

		assert_eq!(actual, 5934);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-06");
		let actual = solution_from(&path).unwrap();
		assert_eq!(actual, 351188);
	}
}
