use thiserror::Error;

mod line;

pub use line::check_line;
pub use line::LineStatus;

#[derive(Debug, Error)]
pub enum ParseLineError
{
	#[error("IO error: {0}!")]
	IoError(#[from] std::io::Error),

	#[error("Line is empty!")]
	EmptyLineError,
}
