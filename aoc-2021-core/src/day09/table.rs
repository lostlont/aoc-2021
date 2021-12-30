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
				let row_width = collected.len() - width;
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

	fn is_valid_position(&self, position: Position) -> bool
	{
		(position.x() < self.width) && (position.y() < self.height)
	}

	fn index_of(&self, position: Position) -> usize
	{
		position.y() * self.width + position.x()
	}
}
