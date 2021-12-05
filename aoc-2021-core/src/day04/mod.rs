mod game;
mod numbers;
mod table;
mod table_position_iterator;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub use game::Game;
pub use numbers::Numbers;
pub use table::Table;
pub use table_position_iterator::TablePositionIterator;

pub fn parse(path: &Path) -> Game
{
	let file = File::open(&path)
		.expect(
			&format!(
				"Couldn't open file {}",
				path.display()));
	let mut reader = BufReader::new(file);

	let mut line = String::new();
	reader
		.read_line(&mut line)
		.expect("Couldn't read line!");
	let numbers_data = line
		.trim()
		.split(',')
		.map(|v| v
			.parse::<i32>()
			.expect(
				&format!("Couldn't parse value: {}!", v)))
		.collect::<Vec<_>>();
	let numbers = Numbers::from(numbers_data);

	let mut tables = vec![];
	let mut table_data = vec![];
	for line in reader.lines()
	{
		let line = line.expect("Couldn't read line!");
		if line.is_empty()
		{
			if !table_data.is_empty()
			{
				tables.push(Table::from(table_data.clone()));
			}
			table_data.clear();
		}
		else
		{
			let row_data = line
				.split(' ')
				.filter(|v| !v.is_empty())
				.map(|v| v
					.parse::<i32>()
					.expect(
						&format!("Couldn't parse value: {}!", v)))
				.collect::<Vec<_>>();

			table_data.push(row_data);
		}
	}
	tables.push(Table::from(table_data.clone()));

	Game::from(numbers, tables)
}
