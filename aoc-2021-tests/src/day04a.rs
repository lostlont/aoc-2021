#[cfg(test)]
mod tests
{
	//use std::path::Path;
	use aoc_2021_core::day04a::
	{
		Table,
	};

	fn create_example_table1() -> Table
	{
		Table::from(vec![
			vec![22, 13, 17, 11, 0],
			vec![8, 2, 23, 4, 24],
			vec![21, 9, 14, 16, 7],
			vec![6, 10, 3, 18, 5],
			vec![1, 12, 20, 15, 19],
		])
	}
}
