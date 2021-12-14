#[cfg(test)]
mod tests
{
	use aoc_2021_core::day05::
	{
		draw_line,
		line,
		Position,
		SparseTable,
	};
	use aoc_2021_core::day05a::
	{
		parse,
	};

	#[test]
	fn example_is_correct()
	{
		let mut table = SparseTable::new();
		let lines = create_example_lines();

		draw_line(&mut table, lines);
		let actual = table
			.into_iter()
			.map(|(_, v)| *v)
			.filter(|v| *v >= 2)
			.count();

		assert_eq!(actual, 5);
	}

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

	fn create_example_lines() -> impl IntoIterator<Item = Position>
	{
		let lines = vec![
			line(
				Position::from(0, 9),
				Position::from(5, 9)),
			line(
				Position::from(8, 0),
				Position::from(0, 8)),
			line(
				Position::from(9, 4),
				Position::from(3, 4)),
			line(
				Position::from(2, 2),
				Position::from(2, 1)),
			line(
				Position::from(7, 0),
				Position::from(7, 4)),

			line(
				Position::from(6, 4),
				Position::from(2, 0)),
			line(
				Position::from(0, 9),
				Position::from(2, 9)),
			line(
				Position::from(3, 4),
				Position::from(1, 4)),
			line(
				Position::from(0, 0),
				Position::from(8, 8)),
			line(
				Position::from(5, 5),
				Position::from(8, 2)),
		];

		lines
			.iter()
			.filter_map(|l| l.as_ref().ok())
			.flatten()
			.cloned()
			.collect::<Vec<_>>()
	}
}
