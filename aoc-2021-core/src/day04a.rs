pub use super::day04::Game;
pub use super::day04::Numbers;
pub use super::day04::Table;

pub fn solution(mut game: Game) -> i32
{
	while !game.is_finished()
	{
		game.step();
	}
	game.score()
}
