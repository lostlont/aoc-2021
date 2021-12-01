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
