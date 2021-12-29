#[cfg(test)]
mod tests
{
	use aoc_2021_core::day08::
	{
		Digit,
		Display,
		Segment,
	};

	#[test]
	fn from_str_parses_values()
	{
		let actual = "ab cd ca | abc def"
			.parse::<Display>()
			.unwrap();

		let expected = Display::new(
			[
				Digit::new([ Segment::A, Segment::B ]),
				Digit::new([ Segment::C, Segment::D ]),
				Digit::new([ Segment::A, Segment::C ]),
			],
			[
				Digit::new([ Segment::A, Segment::B, Segment::C ]),
				Digit::new([ Segment::D, Segment::E, Segment::F ]),
			]);
		assert_eq!(actual, expected);
	}
}
