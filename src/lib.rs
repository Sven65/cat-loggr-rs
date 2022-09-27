pub mod loggr_config;
pub mod level;
use std::fmt;

use owo_colors::{Style, OwoColorize};

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
pub struct LogLevel {
	pub name: String,
	pub style: Style,
}

pub enum Level {
	Fatal(LOG_FATAL_CONFIG),
	Error(LOG_ERROR_CONFIG),
	Warn(LOG_WARN_CONFIG),
	Trace(LOG_TRACE_CONFIG),
	Init(LOG_INIT_CONFIG),
	Info(LOG_INFO_CONFIG),
	Verbose(LOG_VERBOSE_CONFIG),
	Debug(LOG_DEBUG_CONFIG),
}

lazy_static! {
	pub static ref LOG_FATAL_CONFIG: LogLevel = LogLevel   { name: "fatal".to_string(), style: owo_colors::Style::new().red().on_black() };
	pub static ref LOG_ERROR_CONFIG: LogLevel = LogLevel   { name: "error".to_string(), style: owo_colors::Style::new().black().on_red() };
	pub static ref LOG_WARN_CONFIG: LogLevel = LogLevel    { name: "warn".to_string(), style: owo_colors::Style::new().black().on_yellow() };
	pub static ref LOG_TRACE_CONFIG: LogLevel = LogLevel   { name: "trace".to_string(), style: owo_colors::Style::new().green().on_black() };
	pub static ref LOG_INIT_CONFIG: LogLevel = LogLevel    { name: "init".to_string(), style: owo_colors::Style::new().black().on_blue() };
	pub static ref LOG_INFO_CONFIG: LogLevel = LogLevel    { name: "info".to_string(), style: owo_colors::Style::new().black().on_green() };
	pub static ref LOG_VERBOSE_CONFIG: LogLevel = LogLevel { name: "verbose".to_string(), style: owo_colors::Style::new().black().on_cyan() };
	pub static ref LOG_DEBUG_CONFIG: LogLevel = LogLevel   { name: "debug".to_string(), style: owo_colors::Style::new().magenta().on_black() };
}

impl Into<LogLevel> for &LOG_FATAL_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}

impl Into<LogLevel> for &LOG_ERROR_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}


impl Into<LogLevel> for &LOG_WARN_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}


impl Into<LogLevel> for &LOG_TRACE_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}

impl Into<LogLevel> for &LOG_INIT_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}

impl Into<LogLevel> for &LOG_INFO_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}

impl Into<LogLevel> for &LOG_VERBOSE_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}

impl Into<LogLevel> for &LOG_DEBUG_CONFIG {
    fn into(self) -> LogLevel { LogLevel { name: self.name.clone(), style: self.style } } 
}


impl Level {
	pub fn extract(&self) -> LogLevel {
		let level: LogLevel = match self {
			Level::Fatal(level) => level.into(),
			Level::Error(level) => level.into(),
			Level::Warn(level) => level.into(),
			Level::Trace(level) => level.into(),
			Level::Init(level) => level.into(),
			Level::Info(level) => level.into(),
			Level::Verbose(level) => level.into(),
			Level::Debug(level) => level.into(),
		};

		level
	}
}

fn centre_pad(text: &String, length: usize) -> String {
	if text.len() < length {
		let before_count = ((length - text.len()) as f32 / 2 as f32).floor() as usize;
		let after_count = ((length - text.len()) as f32 / 2 as f32).ceil() as usize;

		format!("{}{}{}", " ".repeat(before_count), text, " ".repeat(after_count))
	} else {
		text.to_string()
	}
}

pub fn write(
	args: fmt::Arguments,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
    kvs: Option<&[(&str, &str)]>,
) {

	let log_level = level.extract();
	let centered_str = centre_pad(&log_level.name, 6);

	let level_str = centered_str.style(log_level.style);

	println!("{} {}", level_str , args);

	// let log_level = match level {
	// 	Level::Info(a) => a
	// }

	// let level_string = level.
}

#[macro_export]
macro_rules! log {
	 // log!(target: "my_target", Level::Info; key1 = 42, key2 = true; "a {} event", "log");
	(target: $target:expr, $lvl:expr, $($key:tt = $value:expr),+; $($arg:tt)+) => ({
        $crate::write(
			format_args!($($args)*),
			$lvl,
			&($target, __log_module_path!(), __log_file!(), __log_line!()),
            $crate::__private_api::Option::Some(&[$((__log_key!($key), &$value)),+])
		)
    });

    // log!(target: "my_target", Level::Info; "a {} event", "log");
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
		$crate::write(
			format_args!($($arg)*),
			$lvl,
			// &($target, __log_module_path!(), __log_file!(), __log_line!()),
			&($target, "", "", 0),
			Option::None,
		);
    });

    // log!(Level::Info, "a log event")
    // ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: __log_module_path!(), $lvl, $($arg)+));
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: "", $lvl, $($arg)+));

}

#[macro_export]
macro_rules! log_info {
	// error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // error!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Info, $($arg)+));

	// log_info!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!(LOG_INFO_CONFIG, $($arg)+))
}