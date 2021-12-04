pub use super::day04::Numbers;
pub use super::day04::Table;

pub struct Game
{
	numbers: Numbers,
	tables: Vec<Table>,
	last_number : Option<i32>,
}

impl Game
{
	pub fn from(numbers: Numbers, tables: Vec<Table>) -> Self
	{
		Self
		{
			numbers,
			tables,
			last_number: None,
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
		self.last_number = Some(number);

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

	pub fn score(&self) -> i32
	{
		let table = self.tables
			.iter()
			.filter(|t| t.is_bingo())
			.next()
			.unwrap();

		let sum : i32= table
			.positions()
			.filter(|p| !table.is_marked_at(p.0, p.1))
			.map(|p| table.number_at(p.0, p.1))
			.sum();

		sum * self.last_number.unwrap()
	}
}
