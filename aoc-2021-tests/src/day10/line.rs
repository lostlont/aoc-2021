#[cfg(test)]
mod tests
{
	use aoc_2021_core::day10::
	{
		check_line,
		LineStatus,
	};

	#[test]
	fn check_line_recognizes_simple_valid_line()
	{
		let input = "()";
		let actual = check_line(input);
		assert_eq!(actual, LineStatus::Valid);
	}

	#[test]
	fn check_line_recognizes_complex_valid_line()
	{
		let input = r"{()[(())<><[]>]}";
		let actual = check_line(input);
		assert_eq!(actual, LineStatus::Valid);
	}

	#[test]
	fn check_line_recognizes_simple_incomplete_line()
	{
		let input = "(";
		let actual = check_line(input);
		assert_eq!(actual, LineStatus::Incomplete);
	}

	#[test]
	fn check_line_recognizes_complex_incomplete_line()
	{
		let input = r"()[{{}()}";
		let actual = check_line(input);
		assert_eq!(actual, LineStatus::Incomplete);
	}

	#[test]
	fn check_line_recognizes_simple_corrupted_line()
	{
		let input = "(}";
		let actual = check_line(input);
		assert_eq!(actual, LineStatus::Corrupted('}'));
	}

	#[test]
	fn check_line_recognizes_complex_corrupted_line()
	{
		let input = r"({}{([][>)})";
		let actual = check_line(input);
		assert_eq!(actual, LineStatus::Corrupted('>'));
	}
}
