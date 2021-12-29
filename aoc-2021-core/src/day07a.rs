use super::day07::median;

pub fn solution(input: Vec<i32>) -> Option<i32>
{
	if let Some(median) = median(input.clone())
	{
		input
			.iter()
			.map(|v| (v - median).abs())
			.sum::<i32>()
			.into()
	}
	else
	{
		None
	}
}
