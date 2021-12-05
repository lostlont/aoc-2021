use std::collections::hash_map;
use std::collections::HashMap;
use super::position::Position;

pub struct SparseTable
{
	entries: HashMap<Position, i32>,
}

impl SparseTable
{
	pub fn new() -> Self
	{
		Self
		{
			entries: HashMap::new(),
		}
	}

	pub fn get(&self, position: Position) -> i32
	{
		self.entries.get(&position).cloned().unwrap_or(0)
	}

	pub fn set(&mut self, position: Position, value: i32)
	{
		self.entries.insert(position, value);
	}
}

impl<'a> IntoIterator for &'a SparseTable
{
	type Item = (&'a Position, &'a i32);
	type IntoIter = hash_map::Iter<'a, Position, i32>;

	fn into_iter(self) -> Self::IntoIter
	{
		(&self.entries).into_iter()
	}
}
