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
