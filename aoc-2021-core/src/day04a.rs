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
	data: Vec<i32>,
	marks: Vec<bool>,
	width: i32,
	height: i32,
}

impl Table
{
	pub fn from<TNumbers>(data: TNumbers) -> Self
	where
		TNumbers: IntoIterator + Clone,
		TNumbers::Item: IntoIterator<Item = i32> + Clone,
	{
		Self
		{
			data: data.clone().into_iter().flatten().collect(),
			marks: data.clone().into_iter().flatten().map(|_| false).collect(),
			width: Self::width_of(data.clone()) as i32,
			height: Self::height_of(data) as i32,
		}
	}

	pub fn from_marks<TNumbers, TMarks>(data: TNumbers, marks: TMarks) -> Self
	where
		TNumbers: IntoIterator + Clone,
		TNumbers::Item: IntoIterator<Item = i32> + Clone,
		TMarks: IntoIterator,
		TMarks::Item: IntoIterator<Item = bool>,
	{
		Self
		{
			data: data.clone().into_iter().flatten().collect(),
			marks: marks.into_iter().flatten().collect(),
			width: Self::width_of(data.clone()) as i32,
			height: Self::height_of(data) as i32,
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

	pub fn width(&self) -> i32
	{
		self.width
	}

	pub fn height(&self) -> i32
	{
		self.height
	}

	pub fn positions(&self) -> TablePositionIterator
	{
		TablePositionIterator::new(self)
	}

	pub fn is_marked_at(&self, x: i32, y: i32) -> bool
	{
		let index = self.to_index(x, y);
		self.marks[index]
	}

	pub fn mark(&mut self, x: i32, y: i32)
	{
		let index = self.to_index(x, y);
		self.marks[index] = true;
	}

	pub fn is_bingo(&self) -> bool
	{
		false
	}

	fn to_index(&self, x: i32, y: i32) -> usize
	{
		(self.width * y + x) as usize
	}
}

pub struct TablePositionIterator
{
	width: i32,
	height: i32,
	x: i32,
	y: i32,
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
	type Item = (i32, i32);

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
