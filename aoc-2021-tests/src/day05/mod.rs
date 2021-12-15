mod draw_line;
mod line;
mod position;
mod sparse_table;

#[cfg(test)]
mod tests
{
	use aoc_2021_core::day05::
	{
		parse,
		Position,
	};

	#[test]
	fn parse_is_correct()
	{
		let input = "
419,207 -> 419,109
300,888 -> 803,385";
		let actual = parse(input.as_bytes()).unwrap();

		let expected = vec![
			(Position::from(419, 207), Position::from(419, 109)),
			(Position::from(300, 888), Position::from(803, 385)),
		];
		assert_eq!(actual, expected);
	}
}
