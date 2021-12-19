use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::day06::parse;
use super::day06::ParseError;
use super::day06::Simulation;

pub fn solution(mut simulation: Simulation) -> i32
{
	simulation.run();
	simulation.fish().count() as i32
}

pub fn solution_from(path: &Path) -> Result<i32, ParseError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let input = parse(reader)?;
	let simulation = Simulation::new(input, 8, 6, 80);
	Ok(solution(simulation))
}
