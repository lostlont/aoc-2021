#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day05::
	{
		Position,
	};
	use aoc_2021_core::day05a::
	{
		parse,
		solution,
		solution_from,
	};

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();
		let actual = solution(input);
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

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-05");
		let actual = solution_from(&path).unwrap();

		let expected = 5306;
		assert_eq!(actual, expected);
	}

	fn create_example_input() -> Vec<(Position, Position)>
	{
		vec![
			(
				Position::from(0, 9),
				Position::from(5, 9)),
			(
				Position::from(8, 0),
				Position::from(0, 8)),
			(
				Position::from(9, 4),
				Position::from(3, 4)),
			(
				Position::from(2, 2),
				Position::from(2, 1)),
			(
				Position::from(7, 0),
				Position::from(7, 4)),
			(
				Position::from(6, 4),
				Position::from(2, 0)),
			(
				Position::from(0, 9),
				Position::from(2, 9)),
			(
				Position::from(3, 4),
				Position::from(1, 4)),
			(
				Position::from(0, 0),
				Position::from(8, 8)),
			(
				Position::from(5, 5),
				Position::from(8, 2)),
		]
	}
}
