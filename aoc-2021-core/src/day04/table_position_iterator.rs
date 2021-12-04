use super::Table;

pub struct TablePositionIterator
{
	width: usize,
	height: usize,
	x: usize,
	y: usize,
}

impl TablePositionIterator
{
	pub fn new(table: &Table) -> Self
	{
		Self
		{
			width: table.width(),
			height: table.height(),
			x: 0,
			y: 0,
		}
	}
}

impl Iterator for TablePositionIterator
{
	type Item = (usize, usize);

	fn next(&mut self) -> Option<Self::Item>
	{
		let move_down = self.x / self.width;
		self.x %= self.width;
		self.y += move_down;

		let result = if self.y < self.height
		{
			Some((self.x, self.y))
		}
		else
		{
			None
		};

		self.x += 1;
		result
	}
}
