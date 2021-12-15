use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::day05::draw_line;
use super::day05::line;
use super::day05::parse;
use super::day05::ParseLineError;
use super::day05::Position;
use super::day05::SparseTable;

pub fn solution(input: impl IntoIterator<Item = (Position, Position)>) -> i32
{
	let mut table = SparseTable::new();
	let positions = input
		.into_iter()
		.filter(|(start, end)|
			(start.x() == end.x()) ||
			(start.y() == end.y()))
		.filter_map(|(start, end)|
			line(start, end)
			.ok())
		.flatten()
		.collect::<Vec<_>>();

	draw_line(&mut table, positions);
	table
		.into_iter()
		.map(|(_, v)| *v)
		.filter(|v| *v >= 2)
		.count() as i32
}

pub fn solution_from(path: &Path) -> Result<i32, ParseLineError>
{
	let file = File::open(&path)?;
	let reader = BufReader::new(file);
	let input = parse(reader)?;
	Ok(solution(input))
}
