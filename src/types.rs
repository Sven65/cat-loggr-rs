use std::fmt;

use chrono::{DateTime, Utc};

/// Arguments passed to a [`PostHookCallback`]
#[derive(Debug)]
pub struct PostHookCallbackParams {
	/// The level of the log
	pub level: String,
	/// The final formatted text
	pub text: String,
	/// The timestamp of execution
	pub date: DateTime<Utc>,
	/// The formatted timestamp
	pub timestamp: String,
	/// The shard ID
	pub shard: Option<String>,
	// context:
}

/// Arguments passed to a [`PreHookCallback`]
pub struct PreHookCallbackParams<'a> {
	/// The level of the log
	pub level: String,
	/// The arguments being logged
	pub args: fmt::Arguments<'a>,
	/// The timestamp of execution
	pub date: DateTime<Utc>,
	/// The formatted timestamp
	pub timestamp: String,
	/// The shard ID
	pub shard: Option<String>,
}


/// An argument hook callback function
/// 
/// # Arguments
/// * `args` - The provided argument
/// * `date` - The timestamp of execution
pub type ArgHookCallback = fn(args: Option<fmt::Arguments>, date: DateTime<Utc>) -> Option<String>;

/// A post hook callback function
/// 
/// # Arguments
/// * `params` - The parameters that are sent by the hook
pub type PostHookCallback = fn(params: PostHookCallbackParams) -> Option<String>;

/// A pre hook callback function
/// 
/// # Arguments
/// * `params` - The parameters that are sent by the hook
pub type PreHookCallback = fn(params: PreHookCallbackParams) -> Option<String>;



pub struct LogHooks {
	pub pre: Vec<PreHookCallback>,
	pub arg: Vec<ArgHookCallback>,
	pub post: Vec<PostHookCallback>,
}

impl LogHooks {
	pub fn new () -> Self {
		Self {
			pre: Vec::<PreHookCallback>::new(),
			arg: Vec::<ArgHookCallback>::new(),
			post: Vec::<PostHookCallback>::new(),
		}
	}
}