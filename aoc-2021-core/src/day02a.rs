use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq)]
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

	pub fn x(&self) -> i32
	{
		self.x
	}

	pub fn y(&self) -> i32
	{
		self.y
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction
{
	Forward,
	Up,
	Down,
}

#[derive(Debug, PartialEq)]
pub struct Movement
{
	direction: Direction,
	length: i32,
}

impl Movement
{
	pub fn from_values(direction: Direction, length: i32) -> Self
	{
		Self
		{
			direction,
			length,
		}
	}

	pub fn direction(&self) -> Direction
	{
		self.direction
	}

	pub fn length(&self) -> i32
	{
		self.length
	}
}

pub trait Dive
{
	fn dive(&self, movement: Movement) -> Self;
}

impl Dive for Position
{
	fn dive(&self, movement: Movement) -> Self
	{
		match movement.direction
		{
			Direction::Forward => Position::from_values(
				self.x + movement.length,
				self.y),
			Direction::Up => Position::from_values(
				self.x,
				self.y - movement.length),
			Direction::Down => Position::from_values(
				self.x,
				self.y + movement.length),
		}
	}
}

pub fn parse(value: &str) -> Movement
{
	let parts = value
		.split(' ')
		.collect::<Vec<_>>();
	let direction = parts[0];
	let length = parts[1]
		.parse::<i32>()
		.expect(
			&format!("Couldn't parse value: {}!", parts[1]));

	match direction
	{
		"forward" => Movement::from_values(Direction::Forward, length),
		"up" => Movement::from_values(Direction::Up, length),
		"down" => Movement::from_values(Direction::Down, length),
		_ => panic!(),
	}
}

pub fn dive_all(input: &[&str]) -> Position
{
	input
		.iter()
		.map(|line| parse(line))
		.fold(
			Position::new(),
			|state, movement| state.dive(movement))
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
