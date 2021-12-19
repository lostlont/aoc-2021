use super::day06::Simulation;

pub fn solution(mut simulation: Simulation) -> i32
{
	simulation.run();
	simulation.fish().count() as i32
}
