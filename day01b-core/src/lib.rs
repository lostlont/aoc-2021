use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use day01_core::gradients;
use day01_core::sum;

pub fn window(input: &Vec<i32>) -> Vec<i32>
{
	input
		.iter()
		.enumerate()
		.filter(|(i, _)| *i < input.len() - 2)
		.map(|(i, _)| input
			.iter()
			.skip(i)
			.take(3)
			.sum())
		.collect()
}

pub fn solution(input: &Vec<i32>) -> i32
{
	let averages = window(input);
	let gradients = gradients(&averages);
	sum(&gradients)
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
