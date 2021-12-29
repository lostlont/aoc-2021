#[cfg(test)]
mod tests
{
	use aoc_2021_core::day08::Segment;

	#[test]
	fn from_str_parses_a()
	{
		let actual = "a"
			.parse::<Segment>()
			.unwrap();

		assert_eq!(actual, Segment::A);
	}

	#[test]
	fn from_str_parses_g()
	{
		let actual = "g"
			.parse::<Segment>()
			.unwrap();

		assert_eq!(actual, Segment::G);
	}

	#[test]
	fn from_str_returns_error_for_invalid_string()
	{
		let actual = "x".parse::<Segment>();

		assert!(actual.is_err());
	}
}
