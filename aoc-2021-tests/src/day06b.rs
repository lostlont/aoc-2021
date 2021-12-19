#[cfg(test)]
mod tests
{
	use aoc_2021_core::day06::
	{
		Simulation,
		solution,
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
}
