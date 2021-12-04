#[cfg(test)]
mod tests
{
	//use std::path::Path;
	use aoc_2021_core::day04a::
	{
		Numbers,
	};

	#[test]
	fn numbers_pull_returns_only()
	{
		let mut numbers = Numbers::from(vec![3]);

		let actual = numbers.pull();

		let expected = 3;
		assert_eq!(actual, expected);
	}

	#[test]
	fn numbers_pull_decreases_count()
	{
		let mut numbers = Numbers::from(vec![3]);
		numbers.pull();

		let actual = numbers.count();

		let expected = 0;
		assert_eq!(actual, expected);
	}

	#[test]
	fn numbers_pull_pops_first()
	{
		let mut numbers = Numbers::from(vec![3, 5]);
		numbers.pull();

		let actual = numbers.pull();

		let expected = 5;
		assert_eq!(actual, expected);
	}
}
