#[cfg(test)]
mod tests
{
	use aoc_2021_core::day07::median;

	#[test]
	fn median_returns_none_for_no_element()
	{
		let actual = median::<[_; 0], i32>([]);
		assert_eq!(actual, None);
	}

	#[test]
	fn median_returns_single_element_for_single_element()
	{
		let actual = median([3]);

		let expected = Some(3);
		assert_eq!(actual, expected);
	}

	#[test]
	fn median_returns_sorted_middle_element_for_three_distinct_elements()
	{
		let actual = median([5, 1, 2]);

		let expected = Some(2);
		assert_eq!(actual, expected);
	}

	#[test]
	fn median_returns_sorted_middle_element_for_three_plus_two_elements()
	{
		let actual = median([2, 2, 5, 2, 5]);

		let expected = Some(2);
		assert_eq!(actual, expected);
	}

	#[test]
	fn median_returns_average_for_two_elements()
	{
		let actual = median([2, 5]);

		let expected = Some(3);
		assert_eq!(actual, expected);
	}
}
