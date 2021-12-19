use std::collections::HashMap;
use std::iter::repeat;
use super::Fish;

pub struct FishCount
{
	fish: Fish,
	count: usize,
}

pub struct Simulation
{
	fish_count: Vec<FishCount>,
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
		let mut fish_map = HashMap::new();
		for fish in fish.into_iter()
		{
			let new_amount = fish_map.get(&fish).map_or(1, |v| v + 1);
			fish_map.insert(fish, new_amount);
		}
		Self
		{
			fish_count: fish_map
				.iter()
				.map(|(fish, &amount)| FishCount
				{
					fish: fish.clone(),
					count: amount,
				})
				.collect(),
			new_timer,
			reset_timer,
			remaining_days,
		}
	}

	pub fn fish<'a>(&'a self) -> impl Iterator<Item = &'a Fish>
	{
		self.fish_count
			.iter()
			.flat_map(|e| repeat(&e.fish).take(e.count))
	}

	pub fn fish_count(&self) -> usize
	{
		self.fish_count
			.iter()
			.map(|e| e.count)
			.sum()
	}

	pub fn remaining_days(&self) -> i32
	{
		self.remaining_days
	}

	pub fn step(&mut self)
	{
		let mut reset_index = None;
		let mut respawn_index = None;

		for (index, entry) in &mut self.fish_count.iter_mut().enumerate()
		{
			if entry.fish.timer > 0
			{
				entry.fish.timer -= 1;
			}
			else
			{
				respawn_index = Some(index);
			}

			if entry.fish.timer == self.reset_timer
			{
				reset_index = Some(index);
			}
		}

		if let Some(respawn_index) = respawn_index
		{
			if let Some(reset_index) = reset_index
			{
				self.fish_count[reset_index].count += self.fish_count[respawn_index].count;
			}
			else
			{
				self.fish_count.push(FishCount
					{
						fish: Fish
						{
							timer: self.reset_timer,
						},
						count: self.fish_count[respawn_index].count,
					});
			}

			self.fish_count[respawn_index].fish.timer = self.new_timer;
		}
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
