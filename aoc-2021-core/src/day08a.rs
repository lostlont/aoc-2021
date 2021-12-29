use super::day08::Board;

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
