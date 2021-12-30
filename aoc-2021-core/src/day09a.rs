use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::common::Position;
use super::day09::Table;
use super::day09::ParseTableError;

pub fn solve(table: &Table) -> i32
{
	find_low_points(table)
		.into_iter()
		.map(|(_, v)| v + 1)
		.sum()
}

pub fn solve_from(path: &Path) -> Result<i32, ParseTableError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let table = Table::parse(reader)?;

	Ok(solve(&table))
}

pub fn find_low_points<'a>(table: &'a Table) -> impl IntoIterator<Item = (Position, i32)> + 'a
{
	table
		.into_iter()
		.filter(|(p, v)| neighbors_in(table, *p)
			.into_iter()
			.all(|(_, v2)| *v < v2))
}

fn neighbors_in<'a>(table: &'a Table, position: Position) -> impl IntoIterator<Item = (Position, i32)> + 'a
{
	neighbors(position)
		.into_iter()
		.filter_map(|p| table
			.get(p)
			.map(|v| (p, v)))
}

fn neighbors(position: Position) -> impl IntoIterator<Item = Position>
{
	let mut result = vec![];

	if position.x() > 0
	{
		result.push(Position::from(position.x() - 1, position.y()));
	}

	if position.y() > 0
	{
		result.push(Position::from(position.x(), position.y() - 1));
	}

	result.push(Position::from(position.x() + 1, position.y()));
	result.push(Position::from(position.x(), position.y() + 1));

	result
}
