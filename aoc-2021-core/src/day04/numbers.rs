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
