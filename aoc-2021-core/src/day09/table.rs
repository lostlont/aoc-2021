use super::super::common::Position;

#[derive(Debug, PartialEq)]
pub struct Table
{
	width: usize,
	height: usize,
	values: Vec<i32>,
}

impl Table
{
	pub fn new<TValues>(values: TValues) -> Option<Self>
	where
		TValues: IntoIterator,
		TValues::Item: IntoIterator<Item = i32>,
	{
		let mut width = 0;
		let mut height = 0;
		let mut collected = vec![];

		for row in values.into_iter()
		{
			height += 1;

			collected.extend(row);
			if height == 1
			{
				width = collected.len();
				if width == 0
				{
					return None;
				}
			}
			else
			{
				let row_width = collected.len() - width * (height - 1);
				if row_width != width
				{
					return None;
				}
			}
		}

		if height == 0
		{
			return None;
		}

		Some(Self
		{
			width,
			height,
			values: collected,
		})
	}

	pub fn width(&self) -> usize
	{
		self.width
	}

	pub fn height(&self) -> usize
	{
		self.height
	}

	pub fn get(&self, position: Position) -> Option<i32>
	{
		if self.is_valid_position(position)
		{
			let index = self.index_of(position);
			Some(self.values[index])
		}
		else
		{
			None
		}
	}

	pub fn set(&mut self, position: Position, value: i32)
	{
		if self.is_valid_position(position)
		{
			let index = self.index_of(position);
			self.values[index] = value;
		}
	}

	pub fn is_valid_position(&self, position: Position) -> bool
	{
		(position.x() < self.width) && (position.y() < self.height)
	}

	fn index_of(&self, position: Position) -> usize
	{
		position.y() * self.width + position.x()
	}
}

pub struct TableIterator<'a>
{
	table: &'a Table,
	position: Position,
}

impl<'a> Iterator for TableIterator<'a>
{
	type Item = (Position, i32);

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.position.y() < self.table.height
		{
			let position = self.position;
			let value = self.table
				.get(self.position)
				.unwrap();

			self.position = if self.position.x() < self.table.width - 1
			{
				Position::from(
					self.position.x() + 1,
					self.position.y())
			}
			else
			{
				Position::from(
					0,
					self.position.y() + 1)
			};

			Some((position, value))
		}
		else
		{
			None
		}
	}
}

impl<'a> IntoIterator for &'a Table
{
	type Item = (Position, i32);
	type IntoIter = TableIterator<'a>;

	fn into_iter(self) -> Self::IntoIter
	{
		TableIterator
		{
			table: self,
			position: Position::from(0, 0),
		}
	}
}
