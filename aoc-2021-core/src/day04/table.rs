use super::TablePositionIterator;

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

	pub fn number_at(&self, x: usize, y: usize) -> i32
	{
		let index = self.to_index(x, y);
		self.numbers[index]
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
