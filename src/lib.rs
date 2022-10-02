#![feature(doc_cfg)]

#![doc = include_str!("../README.md")]

use std::{fmt, collections::HashMap, sync::Mutex};
use chrono::{DateTime, Utc};
use lazy_static::{lazy_static,};
pub use loggr_config::LoggrConfig;
use owo_colors::{OwoColorize};

pub use log_level::LogLevel;
use types::{LogHooks, PreHookCallback, ArgHookCallback, PostHookCallback};

use crate::types::{PostHookCallbackParams};

pub mod log_level;
pub mod loggr_config;
pub mod types;


pub struct CatLoggr {
	pub level_map: HashMap<String, LogLevel>,

	max_length: usize,

	timestamp_format: String,
	shard: Option<String>,
	shard_length: Option<usize>,

	levels: Vec::<LogLevel>,

	hooks: LogHooks,

	level_name: Option<String>,

	color_enabled: bool,
}

impl Default for CatLoggr {
	fn default() -> Self {
		Self {
			level_map: Default::default(),
			max_length: Default::default(),
			levels: Default::default(),
			timestamp_format: "%d/%m %H:%M:%S".to_string(),
			shard: None,
			shard_length: None,
			hooks: LogHooks::new(),
			level_name: None,
			color_enabled: true
		}
	}
}

fn top<T: Clone>(vec: &mut Vec<T>) -> Option<T> {
	vec.last().cloned()
}

impl CatLoggr {
	fn get_default_levels() -> Vec<LogLevel> {
		let default_levels: Vec<LogLevel> = vec![
			LogLevel   { name: "fatal".to_string(), style: owo_colors::Style::new().red().on_black(), position: None },
			LogLevel   { name: "error".to_string(), style: owo_colors::Style::new().black().on_red(), position: None },
			LogLevel   { name: "warn".to_string(), style: owo_colors::Style::new().black().on_yellow(), position: None },
			LogLevel   { name: "trace".to_string(), style: owo_colors::Style::new().green().on_black(), position: None },
			LogLevel   { name: "init".to_string(), style: owo_colors::Style::new().black().on_blue(), position: None },
			LogLevel   { name: "info".to_string(), style: owo_colors::Style::new().black().on_green(), position: None},
			LogLevel   { name: "verbose".to_string(), style: owo_colors::Style::new().black().on_cyan(), position: None },
			LogLevel   { name: "debug".to_string(), style: owo_colors::Style::new().magenta().on_black(), position: None }
		];

		default_levels
	}

	#[doc(hidden)]
	pub fn add_pre_hook(&mut self, func: PreHookCallback) -> &mut Self {
		self.hooks.pre.push(func);

		self
	}

	#[doc(hidden)]
	pub fn add_arg_hook(&mut self, func: ArgHookCallback) -> &mut Self {
		self.hooks.arg.push(func);

		self
	}

	/// Adds a post hook, ran after formatting, but before final output
	/// The string returned by the hook will be the text after the level name.
	/// 
	/// # Arguments
	/// * `func` - The function to run
	/// 
	/// # Examples
	/// 
	/// ```rust
	/// use cat_loggr::CatLoggr;
	/// let mut logger = CatLoggr::new(None);
	/// 
	/// logger.add_post_hook(|params| -> Option<String> {
	/// 	let string: String = "New log".to_string();
	/// 	Some(string)
	/// });
	/// ```
	pub fn add_post_hook(&mut self, func: PostHookCallback) -> &mut Self {
		self.hooks.post.push(func);

		self
	}

	/// Configures the logger
	/// 
	/// # Arguments
	/// * `options` - The options to configure with
	/// 
	/// # Examples
	/// 
	/// ```rust
	/// // Configuring the levels
	/// 
	/// use cat_loggr::{CatLoggr, LogLevel, LoggrConfig};
	/// let mut logger = CatLoggr::new(None);
	/// 
	/// logger.config(Some(LoggrConfig {
	/// 	levels: Some(vec![
	/// 		LogLevel   { name: "fatal".to_string(), style: owo_colors::Style::new().red().on_black(), position: None },
	/// 		LogLevel   { name: "info".to_string(), style: owo_colors::Style::new().red().on_black(), position: None }	
	/// 	]),
	///		color_enabled: false,
	/// 	..LoggrConfig::default()
	/// }));
	/// ```
	pub fn config(&mut self, options: Option<LoggrConfig>) -> &mut Self {
		let options = options.unwrap_or_default();		

		if options.timestamp_format.is_some() {
			self.timestamp_format = options.timestamp_format.unwrap();
		}

		if options.shard.is_some() {
			self.shard = options.shard;
		}

		if options.shard_length.is_some() {
			self.shard_length = options.shard_length;
		}

		if options.levels.is_some() {
			self.set_levels(options.levels.unwrap());
		} else {
			self.set_levels(Self::get_default_levels());	
		}

		if options.level.is_some() {
			self.level_name = options.level;
		} else {
			self.level_name = Some(top::<LogLevel>(&mut self.levels).unwrap().name);
		}

		self.color_enabled = options.color_enabled;

		self
	}

	/// Creates an instance of the logger
	/// 
	/// # Arguments
	/// * `options` - The options to instantiate with
	/// 
	/// # Examples
	/// 
	/// ```rust
	/// // Create with levels and shard ID
	/// 
	/// use cat_loggr::{CatLoggr, LoggrConfig, log_level::LogLevel};
	/// let logger = CatLoggr::new(Some(LoggrConfig {
	///		levels: Some(vec![
	/// 		LogLevel   { name: "fatal".to_string(), style: owo_colors::Style::new().red().on_black(), position: None },
	/// 		LogLevel   { name: "info".to_string(), style: owo_colors::Style::new().red().on_black(), position: None }	
	/// 	]),
	/// 	shard: Some("123".to_string()),
	/// 	shard_length: Some(4),
	/// 	..LoggrConfig::default()
	/// }));
	/// ```
	pub fn new(options: Option<LoggrConfig>) -> Self {
		let mut logger = Self::default();
		logger.config(options);	

		logger
	}

	#[doc(hidden)]
	fn get_timestamp(&self, time: Option<DateTime<Utc>>) -> String {
		let now: DateTime<Utc> = time.unwrap_or(Utc::now());

		let format_string = &self.timestamp_format;

		let formatted = now.format(&format_string);

		formatted.to_string()
	}

	/// Overwrites the currently set levels with a custom set
	/// 
	/// # Arguments
	/// 
	/// * `levels` - New levels to override with, in order from high to low priority
	/// 
	/// # Examples
	/// 
	/// ```
	/// use cat_loggr::{CatLoggr, LogLevel};
	/// 
	/// let mut logger = CatLoggr::new(None);
	/// 
	/// logger.set_levels(vec![
	/// 	LogLevel   { name: "fatal".to_string(), style: owo_colors::Style::new().red().on_black(), position: None },
	/// 	LogLevel   { name: "info".to_string(), style: owo_colors::Style::new().red().on_black(), position: None }
	/// ]);
	/// ```
	pub fn set_levels(&mut self, levels: Vec<LogLevel>) -> &Self {
		self.level_map.clear();
		self.levels = levels;

		let mut max = 0;

		for (position, level) in self.levels.iter_mut().enumerate() {
			level.position = Some(position);

			max = if level.name.len() > max {
				level.name.len()
			} else {
				max
			};
			
			if !self.level_map.contains_key(&level.name) {
				self.level_map.insert(level.name.clone(), level.clone());

			}
		}

		self.max_length = max + 2;

		self
	}


	/// Sets the level threshold. Only logs on and above the threshold will be output
	/// 
	/// # Arguments
	/// * `level` - The name of the level threshold
	pub fn set_level(&mut self, level: &str) -> &Self {
		if !self.level_map.contains_key(level) {
			panic!("The level `{}` level doesn't exist.", level);
		}

		self.level_name = Some(level.to_string());
		

		self
	}

	/// Center aligns text
	/// 
	/// # Arguments
	/// 
	/// * `text` - The text to align
	/// * `length` - The length that it should be padded to
	fn centre_pad(text: &String, length: usize) -> String {
		if text.len() < length {
			let before_count = ((length - text.len()) as f32 / 2 as f32).floor() as usize;
			let after_count = ((length - text.len()) as f32 / 2 as f32).ceil() as usize;
	
			format!("{}{}{}", " ".repeat(before_count), text, " ".repeat(after_count))
		} else {
			text.to_string()
		}
	}

	#[doc(hidden)]
	/// Internal function that writes the log by taking [`fmt::Arguments`]
	/// 
	/// # Arguments
	/// * `args` - The text to write
	/// * `level` - The level to write at
	pub fn __write(&self, args: fmt::Arguments, level: &str) {
		self.log(format!("{}", args).as_str(), level);
	}

	#[doc(hidden)]
	fn get_level(&self) -> &LogLevel {
		self.level_map.get(&self.level_name.clone().unwrap()).unwrap()
	}

	/// Writes the log
	/// 
	/// # Arguments
	/// * `text` - The text to write
	/// * `level` - The level to write at
	/// 
	/// # Examples
	/// 
	/// ```rust
	/// // Log to the `info` level
	/// 
	/// use cat_loggr::CatLoggr;
	/// let logger = CatLoggr::new(None);
	/// 
	/// logger.log("This is a log", "info");
	/// ```
	pub fn log(
		&self,
		text: &str,
		level: &str,
	) -> &Self {

		if !self.level_map.contains_key(level) {
			panic!("The level `{}` level doesn't exist.", level);
		}

		let current_log_level = self.get_level();
		let log_level = self.level_map.get(level).unwrap();

		if log_level.position.unwrap() > current_log_level.position.unwrap() {
			return self;
		}

		let shard_text = if self.shard.is_some() {
			CatLoggr::centre_pad(&self.shard.clone().unwrap(), self.shard_length.unwrap())
		} else {
			"".to_string()
		};

		let formatted_shard_text = if self.color_enabled { shard_text.black().on_yellow().to_string() } else { shard_text };
		let centered_str = CatLoggr::centre_pad(&log_level.name, self.max_length);
		let level_str = if self.color_enabled { centered_str.style(log_level.style).to_string()} else { centered_str};

		let now = Utc::now();

		let timestamp = self.get_timestamp(Some(now));
		let formatted_timestamp = if self.color_enabled { timestamp.black().on_white().to_string() } else { timestamp.clone() };
	
		let mut final_text: String = text.to_string();

		for hook in self.hooks.post.iter() {
			let res = hook(PostHookCallbackParams {
				text: text.to_string(),
				date: now,
				timestamp: timestamp.clone(),
				level: level.to_string(),
				shard: self.shard.clone(),
			});

			if res.is_some() {
				final_text = res.unwrap();
			}
		}

		let final_string = format!("{}{}{} {}", formatted_shard_text, formatted_timestamp, level_str, final_text);

		println!("{}", final_string);

		self
	}
}

#[cfg(feature = "macros")]
lazy_static! {
	#[doc(cfg(feature = "macros"))]
	pub static ref CAT_LOGGR: Mutex<CatLoggr> = Mutex::new(CatLoggr::new(None));
}

#[doc(cfg(feature = "macros"))]
#[cfg(feature = "macros")]
mod macros {
	/// Logs something to the console with a specified level, using the default logger.
	/// 
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log;
	/// 
	/// log!("info", "Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log!("info", "Default log {:#?}", data);
	/// ```
	/// 
	///
	#[macro_export]
	macro_rules! log {
		// log!(target: "my_target", Level::Info; key1 = 42, key2 = true; "a {} event", "log");
		(target: $target:expr, $lvl:expr, $($key:tt = $value:expr),+; $($arg:tt)+) => ({
			cat_loggr::CAT_LOGGR.write(
				format_args!($($args)*),
				$lvl,
			)
		});

		// log!(target: "my_target", Level::Info; "a {} event", "log");
		(target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
			cat_loggr::CAT_LOGGR.lock().unwrap().__write(
				format_args!($($arg)*),
				$lvl,
			);
		});

		($lvl:expr, $($arg:tt)+) => ($crate::log!(target: "", $lvl, $($arg)+));

	}

	/// Logs something to the console with the default fatal level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_fatal;
	///
	/// log_fatal!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_fatal!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_fatal {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "fatal", $($arg)+));
		($($arg:tt)+) => ($crate::log!("fatal", $($arg)+))
	}

	/// Logs something to the console with the default error level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_error;
	///
	/// log_error!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_error!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_error {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "error", $($arg)+));
		($($arg:tt)+) => ($crate::log!("error", $($arg)+))
	}

	/// Logs something to the console with the default warn level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_warn;
	///
	/// log_warn!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_warn!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_warn {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "warn", $($arg)+));
		($($arg:tt)+) => ($crate::log!("warn", $($arg)+))
	}

	/// Logs something to the console with the default trace level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_trace;
	///
	/// log_trace!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_trace!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_trace {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "trace", $($arg)+));
		($($arg:tt)+) => ($crate::log!("trace", $($arg)+))
	}

	/// Logs something to the console with the default init level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_init;
	///
	/// log_init!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_init!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_init {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "init", $($arg)+));
		($($arg:tt)+) => ($crate::log!("init", $($arg)+))
	}


	/// Logs something to the console with the default info level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_info;
	///
	/// log_info!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_info!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_info {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "info", $($arg)+));
		($($arg:tt)+) => ($crate::log!("info", $($arg)+))
	}


	/// Logs something to the console with the default verbose level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_verbose;
	///
	/// log_verbose!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_verbose!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_verbose {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "verbose", $($arg)+));
		($($arg:tt)+) => ($crate::log!("verbose", $($arg)+))
	}

	/// Logs something to the console with the default debug level, using the default logger.
	/// 
	/// # Example
	/// 
	/// ```rust
	/// use cat_loggr::log_debug;
	///
	/// log_debug!("Default log");
	/// 
	/// let data = vec!["a", "b", "c"];
	/// 
	/// log_debug!("{:#?}", data);
	/// ```
	#[macro_export]
	macro_rules! log_debug {
		(target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, "debug", $($arg)+));
		($($arg:tt)+) => ($crate::log!("debug", $($arg)+))
	}
}