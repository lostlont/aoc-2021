pub struct Numbers(Vec<i32>);

impl Numbers
{
	pub fn from(numbers: Vec<i32>) -> Self
	{
		Self(numbers.iter().cloned().rev().collect())
	}

	pub fn pull(&mut self) -> i32
	{
		let result = self.0[self.0.len() - 1];
		self.0.pop();
		result
	}

	pub fn count(&self) -> usize
	{
		self.0.len()
	}
}

pub struct Table
{
	numbers: Vec<i32>,
	marks: Vec<bool>,
	width: usize,
	height: usize,
}

impl Table
{
	pub fn from<TNumbers>(numbers: TNumbers) -> Self
	where
		TNumbers: IntoIterator + Clone,
		TNumbers::Item: IntoIterator<Item = i32> + Clone,
	{
		Self
		{
			numbers: numbers.clone().into_iter().flatten().collect(),
			marks: numbers.clone().into_iter().flatten().map(|_| false).collect(),
			width: Self::width_of(numbers.clone()),
			height: Self::height_of(numbers),
		}
	}

	pub fn from_marks<TNumbers, TMarks>(numbers: TNumbers, marks: TMarks) -> Self
	where
		TNumbers: IntoIterator + Clone,
		TNumbers::Item: IntoIterator<Item = i32> + Clone,
		TMarks: IntoIterator,
		TMarks::Item: IntoIterator<Item = bool>,
	{
		Self
		{
			numbers: numbers.clone().into_iter().flatten().collect(),
			marks: marks.into_iter().flatten().collect(),
			width: Self::width_of(numbers.clone()),
			height: Self::height_of(numbers),
		}
	}

	fn width_of<TNumbers>(numbers: TNumbers) -> usize
	where
		TNumbers: IntoIterator + Clone,
		TNumbers::Item: IntoIterator<Item = i32> + Clone,
	{
		numbers.clone().into_iter().next().unwrap().into_iter().count()
	}

	fn height_of<TNumbers>(numbers: TNumbers) -> usize
	where
		TNumbers: IntoIterator + Clone,
		TNumbers::Item: IntoIterator<Item = i32> + Clone,
	{
		numbers.into_iter().count()
	}

	pub fn width(&self) -> usize
	{
		self.width
	}

	pub fn height(&self) -> usize
	{
		self.height
	}

	pub fn positions(&self) -> TablePositionIterator
	{
		TablePositionIterator::new(self)
	}

	pub fn is_marked_at(&self, x: usize, y: usize) -> bool
	{
		let index = self.to_index(x, y);
		self.marks[index]
	}

	pub fn mark(&mut self, x: usize, y: usize)
	{
		let index = self.to_index(x, y);
		self.marks[index] = true;
	}

	pub fn is_bingo(&self) -> bool
	{
		self.has_bingo_row() || self.has_bingo_column()
	}

	fn has_bingo_row(&self) -> bool
	{
		(0..self.height)
			.any(|y| (0..self.width)
				.all(|x| self.is_marked_at(x, y)))
	}

	fn has_bingo_column(&self) -> bool
	{
		(0..self.width)
			.any(|x| (0..self.height)
				.all(|y| self.is_marked_at(x, y)))
	}

	fn to_index(&self, x: usize, y: usize) -> usize
	{
		(self.width * y + x) as usize
	}
}

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
