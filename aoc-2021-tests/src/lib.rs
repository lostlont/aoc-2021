#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::gradients;
	use aoc_2021_core::solution;
	use aoc_2021_core::solution_b;
	use aoc_2021_core::solution_b_from;
	use aoc_2021_core::solution_from;
	use aoc_2021_core::sum;
	use aoc_2021_core::window;


	#[test]
	fn gradients_returns_empty_for_one()
	{
		let input = vec![10];

		let output = gradients(&input);

		let expected = vec![];
		assert_eq!(&output, &expected);
	}

	#[test]
	fn gradients_returns_single_for_two()
	{
		let input = vec![10, 12];

		let output = gradients(&input);

		let expected = vec![2];
		assert_eq!(&output, &expected);
	}

	#[test]
	fn gradients_returns_two_for_three()
	{
		let input = vec![10, 15, 5];

		let output = gradients(&input);

		let expected = vec![5, -10];
		assert_eq!(&output, &expected);
	}

	#[test]
	fn gradients_example()
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

		let output = gradients(&input);

		let expected = vec![
			1,
			8,
			2,
			-10,
			7,
			33,
			29,
			-9,
			3,
		];
		assert_eq!(&output, &expected);
	}

	#[test]
	fn sum_returns_one_for_one_positive()
	{
		let gradients = vec![5, -5];

		let output = sum(&gradients);

		assert_eq!(output, 1);
	}

	#[test]
	fn sum_returns_two_for_two_positive()
	{
		let gradients = vec![5, -5, -8, 3, -11];

		let output = sum(&gradients);

		assert_eq!(output, 2);
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

		let actual = solution(&input);

		assert_eq!(actual, 7);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-01");
		let solution = solution_from(&path);

		assert_eq!(solution, 1602);
	}

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
	fn example_b_is_correct()
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

		let solution = solution_b(&input);

		assert_eq!(solution, 5);
	}

	#[test]
	fn solution_b_is_correct()
	{
		let path = Path::new("../aoc-2021/input-01");
		let solution = solution_b_from(&path);

		assert_eq!(solution, 1633);
	}
}
