#[cfg(test)]
mod tests
{
	use std::path::Path;
	use aoc_2021_core::day02a::Direction;
	use aoc_2021_core::day02a::Dive;
	use aoc_2021_core::day02a::Movement;
	use aoc_2021_core::day02a::Position;
	use aoc_2021_core::day02b::dive_all;
	use aoc_2021_core::day02b::solution;
	use aoc_2021_core::day02b::solution_from;
	use aoc_2021_core::day02b::State;

	#[test]
	fn dive_handles_forward_zero()
	{
		let state = State::new();
		let input = Movement::from_values(Direction::Forward, 0);

		let actual = state.dive(input);

		let expected = State::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_forward_one()
	{
		let state = State::new();
		let input = Movement::from_values(Direction::Forward, 1);

		let actual = state.dive(input);

		let expected = State::from_values(
			Position::from_values(1, 0),
			0);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_up_zero()
	{
		let state = State::new();
		let input = Movement::from_values(Direction::Up, 0);

		let actual = state.dive(input);

		let expected = State::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_up_one()
	{
		let state = State::new();
		let input = Movement::from_values(Direction::Up, 1);

		let actual = state.dive(input);

		let expected = State::from_values(
			state.position(),
			-1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_down_zero()
	{
		let state = State::new();
		let input = Movement::from_values(Direction::Down, 0);

		let actual = state.dive(input);

		let expected = State::new();
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_down_one()
	{
		let state = State::new();
		let input = Movement::from_values(Direction::Down, 1);

		let actual = state.dive(input);

		let expected = State::from_values(
			state.position(),
			1);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_forward_moves_vertically_by_aim()
	{
		let state = State::from_values(
			Position::new(),
			3);
		let input = Movement::from_values(Direction::Forward, 2);

		let actual = state.dive(input);

		let expected = State::from_values(
			Position::from_values(2, 6),
			3);
		assert_eq!(actual, expected);
	}

	#[test]
	fn dive_handles_different_starting_position()
	{
		let state = State::from_values(
			Position::from_values(2, 3),
			2);
		let input = Movement::from_values(Direction::Forward, 2);

		let actual = state.dive(input);

		let expected = State::from_values(
			Position::from_values(4, 7),
			2);
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

		let actual = dive_all(&input).position();

		let expected = Position::from_values(15, 60);
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

		assert_eq!(actual, 900);
	}

	#[test]
	fn solution_is_correct()
	{
		let path = Path::new("../aoc-2021/input-02");
		let solution = solution_from(&path);

		assert_eq!(solution, 1963088820);
	}
}
