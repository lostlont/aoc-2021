#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day02a::Dive;
	use aoc_2021_core::day02a::dive_all;
	use aoc_2021_core::day02a::Position;
	use aoc_2021_core::day02a::solution;
	use aoc_2021_core::day02a::solution_from;

	#[test]
	fn dive_handles_forward_zero()
	{
		let position = Position::new();
		let input = "forward 0";

		let actual = position.dive(input);

		let expected = Position::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_forward_one()
	{
		let position = Position::new();
		let input = "forward 1";

		let actual = position.dive(input);

		let expected = Position::from_values(1, 0);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_up_zero()
	{
		let position = Position::new();
		let input = "up 0";

		let actual = position.dive(input);

		let expected = Position::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_up_one()
	{
		let position = Position::new();
		let input = "up 1";

		let actual = position.dive(input);

		let expected = Position::from_values(0, -1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_down_zero()
	{
		let position = Position::new();
		let input = "down 0";

		let actual = position.dive(input);

		let expected = Position::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_down_one()
	{
		let position = Position::new();
		let input = "down 1";

		let actual = position.dive(input);

		let expected = Position::from_values(0, 1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_different_starting_position()
	{
		let position = Position::from_values(2, 3);
		let input = "forward 2";

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
