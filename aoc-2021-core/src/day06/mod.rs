mod fish;
mod parse;
mod simulation;

pub use fish::Fish;
pub use parse::parse;
pub use parse::ParseError;
pub use simulation::Simulation;

pub fn solution(mut simulation: Simulation) -> i32
{
	simulation.run();
	simulation.fish().count() as i32
}
