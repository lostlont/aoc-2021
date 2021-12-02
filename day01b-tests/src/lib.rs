#[cfg(test)]
mod tests
{
	use std::path::Path;
	use day01b_core::solution;
	use day01b_core::solution_from;
	use day01b_core::window;

	#[test]
	fn window_returns_single_for_three()
	{
		let input = vec![10, 11, 12];

		let actual = window(&input);

		let expected = vec![33];
		assert_eq!(&actual, &expected);
	}

	#[test]
	fn window_returns_two_for_four()
	{
		let input = vec![10, 11, 12, 5];

		let actual = window(&input);

		let expected = vec![33, 28];
		assert_eq!(&actual, &expected);
	}

	#[test]
	fn example_is_correct()
	{
		let input = vec![
			199,
			200,
			208,
			210,
			200,
			207,
			240,
			269,
			260,
			263,
		];

		let solution = solution(&input);

		assert_eq!(solution, 5);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-01");
		let solution = solution_from(&path);

		assert_eq!(solution, 1633);
	}
}
