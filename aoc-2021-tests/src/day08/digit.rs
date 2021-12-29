#[cfg(test)]
mod tests
{
	use aoc_2021_core::day08::
	{
		Digit,
		Segment,
	};

	#[test]
	fn from_str_parses_single_segment()
	{
		let actual = "a"
			.parse::<Digit>()
			.unwrap();

		let expected = Digit::new([Segment::A]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn from_str_parses_multiple_segments()
	{
		let actual = "gbd"
			.parse::<Digit>()
			.unwrap();

		let expected = Digit::new([
			Segment::B,
			Segment::D,
			Segment::G,
		]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn from_str_returns_error_for_incorrect_segment()
	{
		let actual = "x".parse::<Digit>();
		assert!(actual.is_err());
	}
}
