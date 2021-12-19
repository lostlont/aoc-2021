#[cfg(test)]
mod tests
{
	use aoc_2021_core::day06::
	{
		Simulation,
	};
	use aoc_2021_core::day06a::
	{
		solution,
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
}
