#[cfg(test)]
mod tests
{
	use std::path::Path;
	use super::super::day08::tests::
	{
		create_example_input,
		create_simple_example_input,
	};
	use aoc_2021_core::day08::
	{
		Board,
		Display,
	};
	use aoc_2021_core::day08b::
	{
		deduce,
		solution,
		solution_from,
		solve_display,
	};

	#[test]
	fn deduce_is_correct_for_simple_example()
	{
		let input = create_simple_example_input();
		let display = input
			.parse::<Display>()
			.unwrap();

		let actual = deduce(&display);

		let expected = vec![
			(display.signals().nth(8).unwrap(), 0),
			(display.signals().nth(9).unwrap(), 1),
			(display.signals().nth(2).unwrap(), 2),
			(display.signals().nth(3).unwrap(), 3),
			(display.signals().nth(7).unwrap(), 4),
			(display.signals().nth(1).unwrap(), 5),
			(display.signals().nth(6).unwrap(), 6),
			(display.signals().nth(4).unwrap(), 7),
			(display.signals().nth(0).unwrap(), 8),
			(display.signals().nth(5).unwrap(), 9),
		];
		assert_eq!(actual, expected);
	}

	#[test]
	fn simple_example_is_correct()
	{
		let input = create_simple_example_input();
		let display = input
			.parse::<Display>()
			.unwrap();

		let actual = solve_display(&display);

		assert_eq!(actual, 5353);
	}

	#[test]
	fn example_is_correct()
	{
		let input = create_example_input();
		let board = Board::parse(input.as_bytes()).unwrap();

		let actual = solution(&board);

		assert_eq!(actual, 61229);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-08");

		let actual = solution_from(&path)
			.unwrap();

		assert_eq!(actual, 1046281);
	}
}
