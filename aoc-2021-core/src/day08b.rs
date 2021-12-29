use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;
use super::day08::Board;
use super::day08::Digit;
use super::day08::Display;
use super::day08::ParseBoardError;

pub fn solution_from(path: &Path) -> Result<i32, ParseBoardError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let board = Board::parse(reader)?;

	Ok(solution(&board))
}

pub fn solution(board: &Board) -> i32
{
	board
		.displays()
		.map(|display| solve_display(display))
		.sum::<i32>()
}

pub fn solve_display(display: &Display) -> i32
{
	let deduction = deduce(display);

	let mut result = 0;
	let digits = display
		.output()
		.map(|digit| deduction
			.iter()
			.find(|d| d.0 == digit)
			.unwrap()
			.1);
	for digit in digits
	{
		result = result * 10 + digit;
	}
	result
}

pub fn deduce(display: &Display) -> Vec<(&Digit, i32)>
{
	let digit_1 = display
		.signals()
		.find(|digit| digit.segments().count() == 2)
		.unwrap();
	let digit_1_segments : HashSet<_> = digit_1.segments().cloned().collect();

	let digit_7 = display
		.signals()
		.find(|digit| digit.segments().count() == 3)
		.unwrap();

	let digit_4 = display
		.signals()
		.find(|digit| digit.segments().count() == 4)
		.unwrap();
	let digit_4_segments : HashSet<_> = digit_4.segments().cloned().collect();

	let digit_8 = display
		.signals()
		.find(|digit| digit.segments().count() == 7)
		.unwrap();

	let digit_2 = display
		.signals()
		.find(|digit|
			digit.segments().count() == 5 &&
			digit
				.segments()
				.cloned()
				.collect::<HashSet<_>>()
				.intersection(&digit_4_segments)
				.count() == 2)
		.unwrap();
	let digit_2_segments : HashSet<_> = digit_2.segments().cloned().collect();

	let digit_3 = display
		.signals()
		.find(|digit|
			digit.segments().count() == 5 &&
			digit
				.segments()
				.cloned()
				.collect::<HashSet<_>>()
				.intersection(&digit_1_segments)
				.count() == 2)
		.unwrap();

	let digit_9 = display
		.signals()
		.find(|digit|
			digit.segments().count() == 6 &&
			digit
				.segments()
				.cloned()
				.collect::<HashSet<_>>()
				.intersection(&digit_4_segments)
				.count() == 4)
		.unwrap();

	let digit_5 = display
		.signals()
		.find(|digit|
			digit.segments().count() == 5 &&
			digit
				.segments()
				.cloned()
				.collect::<HashSet<_>>()
				.intersection(&digit_2_segments)
				.count() == 3)
		.unwrap();
	let digit_5_segments : HashSet<_> = digit_5.segments().cloned().collect();

	let digit_6 = display
		.signals()
		.find(|digit|
			digit.segments().count() == 6 &&
			digit
				.segments()
				.cloned()
				.collect::<HashSet<_>>()
				.intersection(&digit_1_segments)
				.count() == 1)
		.unwrap();

	let digit_0 = display
		.signals()
		.find(|digit|
			digit.segments().count() == 6 &&
			digit
				.segments()
				.cloned()
				.collect::<HashSet<_>>()
				.intersection(&digit_5_segments)
				.count() == 4)
		.unwrap();

	vec![
		(digit_0, 0),
		(digit_1, 1),
		(digit_2, 2),
		(digit_3, 3),
		(digit_4, 4),
		(digit_5, 5),
		(digit_6, 6),
		(digit_7, 7),
		(digit_8, 8),
		(digit_9, 9),
	]
}
