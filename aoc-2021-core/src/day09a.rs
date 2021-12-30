use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::day09::Table;
use super::day09::ParseTableError;
use super::day09::find_low_points;

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
