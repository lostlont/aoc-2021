use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn gradients(input: &Vec<i32>) -> Vec<i32>
{
	let current = input
		.iter()
		.skip(1);
	let previous = input
		.iter()
		.take(input.len() - 1);
	current
		.zip(previous)
		.map(|(c, p)| c - p)
		.collect::<Vec<_>>()
}

pub fn sum(values: &Vec<i32>) -> i32
{
	values
		.iter()
		.copied()
		.filter(|x| *x > 0)
		.count()
		as i32
}

pub fn solution(input: &Vec<i32>) -> i32
{
	let values = gradients(&input);
	sum(&values)
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
				.parse::<i32>()
				.expect(
					&format!("Couldn't parse value: {}!", line)))
		.collect::<Vec<_>>();

	solution(&input)
}
