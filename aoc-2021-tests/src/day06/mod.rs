mod parse;
mod simulation;

#[cfg(test)]
pub mod tests
{
	use aoc_2021_core::day06::Fish;

	pub fn create_example_input() -> Vec<Fish>
	{
		vec![
			Fish{ timer: 3 },
			Fish{ timer: 4 },
			Fish{ timer: 3 },
			Fish{ timer: 1 },
			Fish{ timer: 2 },
		]
	}
}
