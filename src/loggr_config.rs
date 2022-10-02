use crate::log_level::LogLevel;

pub struct LoggrConfig {
	/// How to format the log timestamp with [`chrono::prelude::DateTime::format`]
	pub timestamp_format: Option<String>,
	/// The shard ID that the logger is on
	pub shard: Option<String>,
	/// The maximum number of characters that a shard can be
	pub shard_length: Option<usize>,

	/// The default log threshold
	pub level: Option<String>,

	/// Custom level definitions
	pub levels: Option<Vec<LogLevel>>,

	pub color_enabled: bool,
}

impl Default for LoggrConfig {
	fn default() -> Self {
		Self {
			timestamp_format: None,
			shard: None,
			shard_length: None,
			level: None,
			levels: None,
			color_enabled: true,
		}
	}
}