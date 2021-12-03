use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use super::day03a::to_decimal;

pub fn o2_generator_rating<T>(input: T) -> i32
where
	T: IntoIterator,
	T::Item: AsRef<Vec<i32>>,
{
	let mut bit_index = 0;
	let mut remaining = input
		.into_iter()
		.map(|line| line.as_ref().clone())
		.collect::<Vec<_>>();
	while remaining.len() > 1
	{
		let sum = remaining
			.iter()
			.map(|line| line[bit_index])
			.sum::<i32>();
		let count = remaining.len() as i32;
		let bit = if sum as f32 >= (count as f32 / 2.0) { 1 } else { 0 };

		let ids = (0..remaining.len())
			.rev()
			.filter(|i| remaining[*i][bit_index] != bit)
			.collect::<Vec<_>>();
		for id in ids
		{
			remaining.swap_remove(id);
		}
		bit_index += 1;
	}

	to_decimal(remaining[0]
		.clone())
}

pub fn co2_scrubber_rating<T>(input: T) -> i32
where
	T: IntoIterator,
	T::Item: AsRef<Vec<i32>>,
{
	let mut bit_index = 0;
	let mut remaining = input
		.into_iter()
		.map(|line| line.as_ref().clone())
		.collect::<Vec<_>>();
	while remaining.len() > 1
	{
		let sum = remaining
			.iter()
			.map(|line| line[bit_index])
			.sum::<i32>();
		let count = remaining.len() as i32;
		let bit = if sum as f32 >= (count as f32 / 2.0) { 0 } else { 1 };

		let ids = (0..remaining.len())
			.rev()
			.filter(|i| remaining[*i][bit_index] != bit)
			.collect::<Vec<_>>();
		for id in ids
		{
			remaining.swap_remove(id);
		}
		bit_index += 1;
	}

	to_decimal(remaining[0]
		.clone())
}

pub fn solution<'a, T>(input: T) -> i32
where
	T: IntoIterator + Clone,
	T::Item: AsRef<Vec<i32>>,
{
	let o2 = o2_generator_rating(input.clone());
	let co2 = co2_scrubber_rating(input);
	o2 * co2
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
	solution(input)
}
