#[cfg(test)]
mod tests
{
	use aoc_2021_core::day07::
	{
		parse,
	};

	#[test]
	fn parse_is_correct()
	{
		let input = "1,2,3".as_bytes();

		let actual = parse(input)
			.unwrap();

		let expected = vec![ 1, 2, 3 ];
		assert_eq!(actual, expected);
	}
}
