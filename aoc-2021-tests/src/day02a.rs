#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day02a::
	{
		Dive,
		dive_all,
		Direction,
		Movement,
		parse,
		Position,
		solution,
		solution_from,
	};

	#[test]
	fn parse_forward()
	{
		let actual = parse("forward 0");

		let expected = Movement::from_values(Direction::Forward, 0);
		assert_eq!(actual, expected);
	}

	#[test]
	fn parse_up()
	{
		let actual = parse("up 1");

		let expected = Movement::from_values(Direction::Up, 1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn parse_down()
	{
		let actual = parse("down 2");

		let expected = Movement::from_values(Direction::Down, 2);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_forward_zero()
	{
		let position = Position::new();
		let input = Movement::from_values(Direction::Forward, 0);

		let actual = position.dive(input);

		let expected = Position::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_forward_one()
	{
		let position = Position::new();
		let input = Movement::from_values(Direction::Forward, 1);

		let actual = position.dive(input);

		let expected = Position::from_values(1, 0);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_up_zero()
	{
		let position = Position::new();
		let input = Movement::from_values(Direction::Up, 0);

		let actual = position.dive(input);

		let expected = Position::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_up_one()
	{
		let position = Position::new();
		let input = Movement::from_values(Direction::Up, 1);

		let actual = position.dive(input);

		let expected = Position::from_values(0, -1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_down_zero()
	{
		let position = Position::new();
		let input = Movement::from_values(Direction::Down, 0);

		let actual = position.dive(input);

		let expected = Position::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_down_one()
	{
		let position = Position::new();
		let input = Movement::from_values(Direction::Down, 1);

		let actual = position.dive(input);

		let expected = Position::from_values(0, 1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_different_starting_position()
	{
		let position = Position::from_values(2, 3);
		let input = Movement::from_values(Direction::Forward, 2);

		let actual = position.dive(input);

		let expected = Position::from_values(4, 3);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_all_is_correct()
	{
		let input = vec![
			"forward 5",
			"down 5",
			"forward 8",
			"up 3",
			"down 8",
			"forward 2",
		];

		let actual = dive_all(&input);

		let expected = Position::from_values(15, 10);
		assert_eq!(actual, expected);
	}

	#[test]
	fn example_is_correct()
	{
		let input = vec![
			"forward 5",
			"down 5",
			"forward 8",
			"up 3",
			"down 8",
			"forward 2",
		];

		let actual = solution(&input);

		assert_eq!(actual, 150);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-02");
		let solution = solution_from(&path);

		assert_eq!(solution, 1714680);
	}
}
