use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::common::Position;
use super::day09::find_low_points;
use super::day09::neighbors_in;
use super::day09::ParseTableError;
use super::day09::Table;

pub fn solve(table: &Table) -> i32
{
	let mut basin_sizes = find_low_points(table)
		.into_iter()
		.map(|(p, _)| find_basin(table, p)
			.into_iter()
			.count() as i32)
		.collect::<Vec<_>>();

	basin_sizes.sort_unstable();

	basin_sizes
		.iter()
		.rev()
		.take(3)
		.product()
}

pub fn solve_from(path: &Path) -> Result<i32, ParseTableError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let table = Table::parse(reader)?;

	Ok(solve(&table))
}

pub fn find_basin<'a>(table: &'a Table, position: Position) -> impl IntoIterator<Item = Position> + 'a
{
	let mut result = HashSet::new();

	if table.is_valid_position(position)
	{
		let mut queue = vec![position];

		while let Some(position) = queue.pop()
		{
			let value = table
				.get(position)
				.unwrap();
			let neighbors = neighbors_in(table, position)
				.into_iter()
				.filter(|(_, v)| (*v != 9) && (value < *v))
				.map(|(p, _)| p);

			result.insert(position);
			queue.extend(neighbors);
		}
	}

	result
}
