use super::Fish;

pub struct Simulation
{
	fish: Vec<Fish>,
	new_timer: i32,
	reset_timer: i32,
	remaining_days: i32,
}

impl Simulation
{
	pub fn new(
		fish: impl IntoIterator<Item = Fish>,
		new_timer: i32,
		reset_timer: i32,
		remaining_days: i32) -> Self
	{
		Self
		{
			fish: fish.into_iter().collect(),
			new_timer,
			reset_timer,
			remaining_days,
		}
	}

	pub fn fish<'a>(&'a self) -> impl Iterator<Item = &'a Fish>
	{
		self.fish.iter()
	}

	pub fn remaining_days(&self) -> i32
	{
		self.remaining_days
	}

	pub fn step(&mut self)
	{
		let mut new_fish = vec![];

		for fish in &mut self.fish
		{
			if fish.timer > 0
			{
				fish.timer -= 1;
			}
			else
			{
				fish.timer = self.reset_timer;
				new_fish.push(Fish
					{
						timer: self.new_timer,
					});
			}
		}

		self.fish.append(&mut new_fish);
	}

	pub fn run(&mut self)
	{
		while self.remaining_days > 0
		{
			self.step();
			self.remaining_days -= 1;
		}
	}
}
