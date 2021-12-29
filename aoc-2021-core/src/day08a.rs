use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::day08::Board;
use super::day08::ParseBoardError;

pub fn solution(board: &Board) -> i32
{
	board
		.displays()
		.flat_map(|display| display.output())
		.filter(|digit|
			{
				match digit.segments().count()
				{
					2 | 3 | 4 | 7 => true,
					_ => false,
				}
			})
		.count()
		as i32
}

pub fn solution_from(path: &Path) -> Result<i32, ParseBoardError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let board = Board::parse(reader)?;

	Ok(solution(&board))
}
