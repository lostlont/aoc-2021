use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn gamma<'a, T>(input: T) -> i32
where
	T: IntoIterator<Item = &'a Vec<i32>>,
{
	let mut state : Option<Vec<i32>> = Option::None;
	let mut count = 0;
	for line in input.into_iter()
	{
		match state.as_mut()
		{
			Some(state) => for (i, v) in line.iter().enumerate()
			{
				state[i] += *v;
			},
			None => state = Some(line.clone()),
		}
		count += 1;
	}

	let half = count / 2;
	let state = state.unwrap();
	let digits = state
		.iter()
		.cloned()
		.map(|v| if v > half { 1 } else { 0 });

	to_decimal(digits)
}

pub fn epsilon<'a, T>(input: T) -> i32
where
	T: IntoIterator<Item = &'a Vec<i32>>,
{
	let mut state : Option<Vec<i32>> = Option::None;
	let mut count = 0;
	for line in input.into_iter()
	{
		match state.as_mut()
		{
			Some(state) => for (i, v) in line.iter().enumerate()
			{
				state[i] += *v;
			},
			None => state = Some(line.clone()),
		}
		count += 1;
	}

	let half = count / 2;
	let state = state.unwrap();
	let digits = state
		.iter()
		.cloned()
		.map(|v| if v > half { 0 } else { 1 });

	to_decimal(digits)
}

pub fn solution<'a, T>(input: T) -> i32
where
	T: IntoIterator<Item = &'a Vec<i32>> + Clone,
{
	let gamma = gamma(input.clone());
	let epsilon = epsilon(input);
	gamma * epsilon
}

pub fn solution_from(path: &Path) -> i32
{
	let file = File::open(&path)
		.expect(
			&format!(
				"Couldn't open file {}",
				path.display()));
	let reader = BufReader::new(file);
	let input = reader
		.lines()
		.map(
			|line| line.expect("Couldn't read line!"))
		.map(
			|line| line
				.chars()
				.map(|c| c
					.to_digit(10)
					.expect(
						&format!(
							"Couldn't parse value: {}!",
							c))
					as i32)
				.collect::<Vec<_>>())
		.collect::<Vec<_>>();
	solution(&input)
}

fn to_decimal<'a, T>(digits: T) -> i32
where
	T: IntoIterator<Item = i32>,
{
	digits
		.into_iter()
		.fold(0, |a, b| (a << 1) + b)
}
