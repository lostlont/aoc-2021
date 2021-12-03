use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use super::day02a::Direction;
use super::day02a::Dive;
use super::day02a::Movement;
use super::day02a::parse;
use super::day02a::Position;

#[derive(Debug, PartialEq)]
pub struct State
{
	position: Position,
	aim: i32,
}

impl State
{
	pub fn new() -> Self
	{
		Self
		{
			position: Position::new(),
			aim: 0,
		}
	}

	pub fn from_values(position: Position, aim: i32) -> Self
	{
		Self
		{
			position,
			aim,
		}
	}

	pub fn position(&self) -> Position
	{
		self.position
	}
}

impl Dive for State
{
	fn dive(&self, movement: Movement) -> Self
	{
		match movement.direction()
		{
			Direction::Forward => State
			{
				position: Position::from_values(
					self.position.x() + movement.length(),
					self.position.y() + self.aim * movement.length()),
				aim: self.aim,
			},
			Direction::Up => State
			{
				position: self.position,
				aim: self.aim - movement.length(),
			},
			Direction::Down => State
			{
				position: self.position,
				aim: self.aim + movement.length(),
			},
		}
	}
}

pub fn dive_all(input: &[&str]) -> State
{
	input
		.iter()
		.map(|line| parse(line))
		.fold(
			State::new(),
			|state, movement| state.dive(movement))
}

pub fn solution(input: &Vec<&str>) -> i32
{
	let state = dive_all(input);
	state.position().x() * state.position().y()
}

pub fn solution_from(path: &Path) -> i32
{
	let file = File::open(&path)
		.expect(
			&format!(
				"Couldn't open file {}",
				path.display()));
	let reader = BufReader::new(file);
	let input = reader
		.lines()
		.map(
			|line| line.expect("Couldn't read line!"))
		.collect::<Vec<_>>();
	let input = input
		.iter()
		.map(|line| &line[..])
		.collect::<Vec<_>>();
	solution(&input)
}
