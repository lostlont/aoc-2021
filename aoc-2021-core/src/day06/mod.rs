mod fish;
mod parse;
mod simulation;

pub use fish::Fish;
pub use parse::parse;
pub use parse::ParseError;
pub use simulation::Simulation;

pub fn solution(mut simulation: Simulation) -> usize
{
	simulation.run();
	simulation.fish_count()
}
