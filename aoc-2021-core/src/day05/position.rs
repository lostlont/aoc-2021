#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position
{
	x: usize,
	y: usize,
}

impl Position
{
	pub fn from(x: usize, y: usize) -> Self
	{
		Self
		{
			x,
			y,
		}
	}

	pub fn x(&self) -> usize
	{
		self.x
	}

	pub fn y(&self) -> usize
	{
		self.y
	}
}
