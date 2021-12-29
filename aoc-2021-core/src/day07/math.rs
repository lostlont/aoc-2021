use std::ops::Add;
use std::ops::Div;

pub fn median<TInput, TItem>(input: TInput) -> Option<TItem>
where
	TItem: Add + Clone + PartialOrd,
	<TItem as Add>::Output: Div<TItem>,
	<<TItem as Add>::Output as Div<TItem>>::Output: Into<TItem>,
	TInput: IntoIterator<Item = TItem>,
	i32: Into<TItem>,
{
	let mut sorted = input
		.into_iter()
		.collect::<Vec<_>>();

	if sorted.is_empty()
	{
		None
	}
	else
	{
		sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

		let count = sorted.len();
		if count % 2 == 0
		{
			let half_count = count / 2;
			let left = sorted[half_count - 1].clone();
			let right = sorted[half_count].clone();
			let sum = left + right;
			let result = sum / 2.into();
			Some(result.into())
		}
		else
		{
			Some(sorted[count / 2].clone())
		}
	}
}
