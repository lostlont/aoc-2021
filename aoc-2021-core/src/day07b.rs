use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::day07::parse;
use super::day07::ParseError;

pub fn solution(input: Vec<i32>) -> Option<i32>
{
	if input.is_empty()
	{
		None
	}
	else
	{
		let sum = input
			.iter()
			.sum::<i32>();
		let average = (sum as f32) / (input.len() as f32);
		let rounded_average = average.round() as i32;

		let mut solution = solution_at(input.iter().cloned(), rounded_average);
		let mut left_solution = solution_at(input.iter().cloned(), rounded_average - 1);
		if left_solution < solution
		{
			let mut position = rounded_average - 1;
			while left_solution < solution
			{
				position -= 1;
				solution = left_solution;
				left_solution = solution_at(input.iter().cloned(), position);
			}
		}
		else
		{
			let mut right_solution = solution_at(input.iter().cloned(), rounded_average + 1);
			if right_solution < solution
			{
				let mut position = rounded_average + 1;
				while right_solution < solution
				{
					position += 1;
					solution = right_solution;
					right_solution = solution_at(input.iter().cloned(), position);
				}
			}
		}
		solution
	}
}

fn solution_at<T>(input: T, position: i32) -> Option<i32>
where
	T: IntoIterator<Item = i32> + Clone,
{
	let count = input.clone().into_iter().count();
	if count > 0
	{
		let result = input
			.into_iter()
			.map(|v| (v - position).abs())
			.map(|d| (1 + d) * d / 2)
			.sum::<i32>();
		Some(result)
	}
	else
	{
		None
	}
}

pub fn solution_from(path: &Path) -> Result<i32, ParseError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let input = parse(reader)?;

	solution(input)
		.ok_or(ParseError::EmptyFileError)
}
