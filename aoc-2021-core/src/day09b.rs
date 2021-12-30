use std::collections::HashSet;
use super::common::Position;
use super::day09::Table;
use super::day09::neighbors_in;

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
