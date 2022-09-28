use owo_colors::Style;

#[derive(Debug, Clone)]
pub struct LogLevel {
	/// The name of the level
	pub name: String,
	/// Style to use when outputting the level
	pub style: Style,
}

impl LogLevel {
	pub fn new(name: String, style: Style) -> Self {
		Self { name, style }
	}
}