mod line;

pub use line::check_line;
pub use line::LineStatus;

pub fn character_points(ch: char) -> i32
{
	match ch
	{
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => 0,
	}
}
