#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day03b::
	{
		co2_scrubber_rating,
		o2_generator_rating,
		solution,
		solution_from,
	};

	#[test]
	fn o2_generator_rating_for_single_line()
	{
		let input = vec![vec![0, 0, 1, 0, 0]];
		let actual = o2_generator_rating(input);

		let expected = 4;
		assert_eq!(actual, expected);
	}

	#[test]
	fn o2_generator_rating_prefers_one()
	{
		let input = vec![
			vec![1],
			vec![0],
		];
		let actual = o2_generator_rating(input);

		let expected = 1;
		assert_eq!(actual, expected);
	}

	#[test]
	fn o2_generator_rating_chooses_most_common_per_bit()
	{
		let input = vec![
			vec![1, 0, 1],
			vec![0, 0, 1],
			vec![1, 1, 0],
			vec![1, 1, 1],
			vec![0, 1, 0],
		];
		let actual = o2_generator_rating(input);

		let expected = 4 + 2 + 1;
		assert_eq!(actual, expected);
	}

	#[test]
	fn o2_generator_rating_is_correct_for_example()
	{
		let input = create_example_input();
		let actual = o2_generator_rating(input);

		let expected = 23;
		assert_eq!(actual, expected);
	}

	#[test]
	fn co2_scrubber_rating_for_single_line()
	{
		let input = vec![vec![0, 0, 1, 0, 0]];
		let actual = co2_scrubber_rating(input);

		let expected = 4;
		assert_eq!(actual, expected);
	}

	#[test]
	fn co2_scrubber_rating_prefers_one()
	{
		let input = vec![
			vec![1],
			vec![0],
		];
		let actual = co2_scrubber_rating(input);

		let expected = 0;
		assert_eq!(actual, expected);
	}

	#[test]
	fn co2_scrubber_rating_chooses_least_common_per_bit()
	{
		let input = vec![
			vec![1, 0, 1],
			vec![0, 0, 1],
			vec![1, 1, 0],
			vec![1, 1, 1],
			vec![0, 1, 0],
		];
		let actual = co2_scrubber_rating(input);

		let expected = 1;
		assert_eq!(actual, expected);
	}

	#[test]
	fn co2_scrubber_rating_is_correct_for_example()
	{
		let input = create_example_input();
		let actual = co2_scrubber_rating(input);

		let expected = 10;
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

		let expected = 230;
		assert_eq!(actual, expected);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-03");
		let actual = solution_from(&path);

		let expected = 6124992;
		assert_eq!(actual, expected);
	}
}
