#[cfg(test)]
mod tests
{
	use aoc_2021_core::day06::
	{
		Fish,
		Simulation,
	};

	#[test]
	fn step_decreases_timer_of_fish()
	{
		let mut subject = Simulation::new(
			vec![
				Fish
				{
					timer: 5,
				},
			],
			15,
			10,
			0,
		);

		subject.step();

		let fish = subject.fish().next().unwrap();
		assert_eq!(fish.timer, 4);
	}

	#[test]
	fn step_resets_timer_of_fish_when_zero_days_left()
	{
		let mut subject = Simulation::new(
			vec![
				Fish
				{
					timer: 0,
				},
			],
			15,
			10,
			0,
		);

		subject.step();

		let fish = subject.fish().next().unwrap();
		assert_eq!(fish.timer, 10);
	}

	#[test]
	fn step_creates_new_fish_when_zero_days_left()
	{
		let mut subject = Simulation::new(
			vec![
				Fish
				{
					timer: 0,
				},
			],
			15,
			10,
			0,
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

	#[test]
	fn fish_count_returns_amount_of_fish()
	{
		let subject = Simulation::new(
			vec![
				Fish
				{
					timer: 0,
				},
				Fish
				{
					timer: 0,
				},
			],
			15,
			10,
			0,
		);

		let actual = subject.fish_count();

		assert_eq!(actual, 2);
	}

	#[test]
	fn run_calls_step_on_each_day()
	{
		let mut subject = Simulation::new(
			vec![
				Fish
				{
					timer: 3,
				},
			],
			15,
			10,
			2,
		);

		subject.run();
		let actual = subject.fish().next().unwrap().timer;

		assert_eq!(actual, 1);
	}

	#[test]
	fn run_runs_until_remaining_days_reached_zero()
	{
		let mut subject = Simulation::new(
			vec![
				Fish
				{
					timer: 3,
				},
			],
			15,
			10,
			2,
		);

		subject.run();
		let actual = subject.remaining_days();

		assert_eq!(actual, 0);
	}
}
