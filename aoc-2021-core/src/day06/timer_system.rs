use super::Fish;

pub struct TimerSystem
{
	fish: Vec<Fish>,
	new_timer: i32,
	reset_timer: i32,
}

impl TimerSystem
{
	pub fn new(fish: impl IntoIterator<Item = Fish>, new_timer: i32, reset_timer: i32) -> Self
	{
		Self
		{
			fish: fish.into_iter().collect(),
			new_timer,
			reset_timer,
		}
	}

	pub fn fish<'a>(&'a self) -> impl Iterator<Item = &'a Fish>
	{
		self.fish.iter()
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
}
