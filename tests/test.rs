use cat_loggr::{log_fatal, log_error, log_warn, log_trace, log_init, log_info, log_verbose, log_debug, loggr_config::LoggrConfig};

#[derive(Debug)]
struct Data {
	pub name: String
}

#[test]
fn test_log() {
	let data = Data {
		name: "Struct Name".to_string(),
	};

	let logger = cat_loggr::CatLoggr::new(None);

	logger.log("test 123", "info")
		.log("test 321", "fatal");

	cat_loggr::CAT_LOGGR.lock().unwrap().config(LoggrConfig {
		shard: Some("123".to_string()),
		shard_length: Some(4),
		..LoggrConfig::default()
	});

	log_fatal!("Fatal log");
	log_fatal!("Fatal log {:#?}", data);

	log_error!("Error log");
	log_error!("Error log {:#?}", data);

	log_warn!("Warn log");
	log_warn!("Warn log {:#?}", data);

	log_trace!("Trace log");
	log_trace!("Trace log {:#?}", data);

	log_init!("Init log");
	log_init!("Init log {:#?}", data);

	log_info!("Info log");
	log_info!("Info log {:#?}", data);

	log_verbose!("Verbose log");
	log_verbose!("Verbose log {:#?}", data);

	log_debug!("Debug log");
	log_debug!("Debug log {:#?}", data);


	// log_info!("Test");
	// log_info!("{:#?}", data);
}