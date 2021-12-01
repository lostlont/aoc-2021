#[cfg(test)]
mod tests
{
	//use day01_core::sum;
	use day01b_core::solution;
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
}
