#[derive(Debug, PartialEq)]
pub enum LineStatus
{
	Valid,
	Corrupted(char),
	Incomplete(Vec<char>),
}

pub fn check_line(line: &str) -> LineStatus
{
	let mut stack = vec![];

	for ch in line.chars()
	{
		if ['(', '[', '{', '<'].contains(&ch)
		{
			stack.push(ch);
		}
		else
		{
			let expected_ch = match stack.last().unwrap()
			{
				'(' => ')',
				'[' => ']',
				'{' => '}',
				'<' => '>',
				_ => panic!(),
			};

			if ch == expected_ch
			{
				stack.pop();
			}
			else
			{
				return LineStatus::Corrupted(ch);
			}
		}
	}

	return if stack.is_empty()
	{
		LineStatus::Valid
	}
	else
	{
		LineStatus::Incomplete(stack)
	}
}
