use super::common::Position;
use super::day09::Table;

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
