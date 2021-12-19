#[cfg(test)]
mod tests
{
	use aoc_2021_core::day06::
	{
		Fish,
		TimerSystem,
	};

	#[test]
	fn step_decreases_timer_of_fish()
	{
		let mut subject = TimerSystem::new(
			vec![
				Fish
				{
					timer: 5,
				},
			],
			15,
			10,
		);

		subject.step();

		let fish = subject.fish().next().unwrap();
		assert_eq!(fish.timer, 4);
	}

	#[test]
	fn step_resets_timer_of_fish_when_zero_days_left()
	{
		let mut subject = TimerSystem::new(
			vec![
				Fish
				{
					timer: 0,
				},
			],
			15,
			10,
		);

		subject.step();

		let fish = subject.fish().next().unwrap();
		assert_eq!(fish.timer, 10);
	}

	#[test]
	fn step_creates_new_fish_when_zero_days_left()
	{
		let mut subject = TimerSystem::new(
			vec![
				Fish
				{
					timer: 0,
				},
			],
			15,
			10,
		);

		subject.step();
		let actual = subject
			.fish()
			.collect::<Vec<_>>();

		let expected = vec![
			Fish
			{
				timer: 10,
			},
			Fish
			{
				timer: 15,
			},
		];
		let expected = expected
			.iter()
			.map(|f| f)
			.collect::<Vec<_>>();
		assert_eq!(actual, expected);
	}
}
