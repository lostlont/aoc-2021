pub use super::day04::Numbers;
pub use super::day04::Table;

pub struct Game
{
	numbers: Numbers,
	tables: Vec<Table>,
}

impl Game
{
	pub fn from(numbers: Numbers, tables: Vec<Table>) -> Self
	{
		Self
		{
			numbers,
			tables,
		}
	}

	pub fn numbers(&self) -> &Numbers
	{
		&self.numbers
	}

	pub fn tables(&self) -> &Vec<Table>
	{
		&self.tables
	}

	pub fn is_finished(&self) -> bool
	{
		self.tables.iter().any(Table::is_bingo)
	}

	pub fn step(&mut self)
	{
		let number = self.numbers.pull();

		for table in &mut self.tables
		{
			let (x, y) = table
				.positions()
				.filter(|p| table.number_at(p.0, p.1) == number)
				.next()
				.unwrap();
			table.mark(x, y);
		}
	}
}
