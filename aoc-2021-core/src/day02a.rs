use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Position
{
	x: i32,
	y: i32,
}

impl Position
{
	pub fn new() -> Self
	{
		Self
		{
			x: 0,
			y: 0,
		}
	}

	pub fn from_values(x: i32, y: i32) -> Self
	{
		Self
		{
			x,
			y,
		}
	}
}

pub trait Dive
{
	fn dive(&self, direction: &str) -> Self;
}

impl Dive for Position
{
	fn dive(&self, vector: &str) -> Self
	{
		let parts = vector
			.split(' ')
			.collect::<Vec<_>>();
		let direction = parts[0];
		let length = parts[1]
			.parse::<i32>()
			.expect(
				&format!("Couldn't parse value: {}!", parts[1]));

		if direction == "forward"
		{
			Position::from_values(self.x + length, self.y)
		}
		else if direction == "up"
		{
			Position::from_values(self.x, self.y - length)
		}
		else if direction == "down"
		{
			Position::from_values(self.x, self.y + length)
		}
		else
		{
			panic!();
		}
	}
}

pub fn dive_all(input: &[&str]) -> Position
{
	input
		.iter()
		.fold(
			Position::new(),
			|state, &line| state.dive(line))
}

pub fn solution(input: &Vec<&str>) -> i32
{
	let position = dive_all(input);
	position.x * position.y
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
