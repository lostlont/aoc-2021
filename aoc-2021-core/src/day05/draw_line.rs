use super::Position;
use super::SparseTable;

pub fn draw_line<T>(table: &mut SparseTable, positions: T)
where
	T: IntoIterator<Item = Position>,
{
	positions
		.into_iter()
		.for_each(|p| increment(table, p));
}

fn increment(table: &mut SparseTable, position: Position)
{
	let value = table.get(position);
	table.set(position, value + 1);
}
