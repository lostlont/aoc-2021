use super::Numbers;
use super::Table;

pub struct Game
{
	numbers: Numbers,
	tables: Vec<Table>,
	last_number : Option<i32>,
	finished_table_indices: Vec<usize>,
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
			finished_table_indices: vec![],
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

	pub fn is_all_finished(&self) -> bool
	{
		self.tables.iter().all(Table::is_bingo)
	}

	pub fn step(&mut self)
	{
		let number = self.numbers.pull();
		self.last_number = Some(number);

		for table in &mut self.tables
		{
			table
				.positions()
				.filter(|p| table.number_at(p.0, p.1) == number)
				.collect::<Vec<_>>()
				.iter()
				.for_each(|p| table.mark(p.0, p.1));
		}

		self.tables
			.iter()
			.enumerate()
			.filter(|(_, table)| table.is_bingo())
			.filter(|(i, _)| !self.finished_table_indices.contains(i))
			.collect::<Vec<_>>()
			.iter()
			.for_each(|(i, _)| self.finished_table_indices.push(*i));
	}

	pub fn score(&self) -> i32
	{
		let table_index = self.finished_table_indices.last().unwrap();
		let table = &self.tables[*table_index];

		let sum : i32 = table
			.positions()
			.filter(|p| !table.is_marked_at(p.0, p.1))
			.map(|p| table.number_at(p.0, p.1))
			.sum();

		sum * self.last_number.unwrap()
	}
}
