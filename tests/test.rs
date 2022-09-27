use cat_loggr::log_info;

#[derive(Debug)]
struct Data {
	pub name: String
}

#[test]
fn test_log() {
	let data = Data {
		name: "Struct Name".to_string(),
	};

	log_info!("Test");
	log_info!("{:#?}", data);
}