#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day03a::
	{
		gamma,
		epsilon,
		solution,
		solution_from,
	};

	#[test]
	fn gamma_returns_most_common_values_in_decimal_for_single_line()
	{
		let input = vec![vec![0, 0, 1, 0, 0]];
		let actual = gamma(input);

		let expected = 4;
		assert_eq!(actual, expected);
	}

	#[test]
	fn gamma_returns_most_common_values_in_decimal_for_multiple_lines()
	{
		let input = vec![
			vec![1, 0, 0],
			vec![0, 0, 1],
			vec![1, 1, 1],
			vec![1, 0, 1],
			vec![0, 0, 1],
		];
		let actual = gamma(input);

		let expected = 4 + 1;
		assert_eq!(actual, expected);
	}

	#[test]
	fn epsilon_returns_least_common_values_in_decimal_for_single_line()
	{
		let input = vec![vec![0, 0, 1, 0, 0]];
		let actual = epsilon(input);

		let expected = 16 + 8 + 2 + 1;
		assert_eq!(actual, expected);
	}

	#[test]
	fn epsilon_returns_least_common_values_in_decimal_for_multiple_lines()
	{
		let input : Vec<Vec<i32>> = vec![
			vec![1, 0, 0],
			vec![0, 0, 1],
			vec![1, 1, 1],
			vec![1, 0, 1],
			vec![0, 0, 1],
		];
		let actual = epsilon(input);

		let expected = 2;
		assert_eq!(actual, expected);
	}

	fn create_example_input() -> Vec<Vec<i32>>
	{
		vec![
			vec![0, 0, 1, 0, 0],
			vec![1, 1, 1, 1, 0],
			vec![1, 0, 1, 1, 0],
			vec![1, 0, 1, 1, 1],
			vec![1, 0, 1, 0, 1],
			vec![0, 1, 1, 1, 1],
			vec![0, 0, 1, 1, 1],
			vec![1, 1, 1, 0, 0],
			vec![1, 0, 0, 0, 0],
			vec![1, 1, 0, 0, 1],
			vec![0, 0, 0, 1, 0],
			vec![0, 1, 0, 1, 0],
		]
	}

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();
		let actual = solution(input);

		let expected = 198;
		assert_eq!(actual, expected);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-03");
		let actual = solution_from(&path);

		let expected = 1071734;
		assert_eq!(actual, expected);
	}
}
