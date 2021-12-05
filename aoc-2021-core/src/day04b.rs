use std::path::Path;

pub use super::day04::Game;
pub use super::day04::parse;

pub fn solution(mut game: Game) -> i32
{
	while !game.is_all_finished()
	{
		game.step();
	}
	game.score()
}

pub fn solution_from(path: &Path) -> i32
{
	solution(parse(path))
}
