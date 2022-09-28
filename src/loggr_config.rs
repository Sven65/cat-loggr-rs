pub struct LoggrConfig {
	pub timestamp_format: Option<String>,
	pub shard: Option<String>,
	pub shard_length: Option<usize>,
}

impl Default for LoggrConfig {
	fn default() -> Self {
		Self {
			timestamp_format: None,
			shard: None,
			shard_length: None,
		}
	}
}