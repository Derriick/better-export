use chrono::prelude::*;
use env_logger;
use log::{debug, error, info, LevelFilter};
use std::{path::Path, path::PathBuf};

use better_export::*;

fn main() {
	let matches = create_app().get_matches();

	let level_filter = match matches.occurrences_of("verbose") {
		//0 => LevelFilter::Error, // error
		0 => LevelFilter::Warn,  // error + warn
		1 => LevelFilter::Info,  // error + warn + info
		2 => LevelFilter::Debug, // error + warn + info + debug
		_ => LevelFilter::Trace, // error + warn + info + debug + trace
	};
	env_logger::builder().filter_level(level_filter).init();

	let conf = match get_conf(&matches) {
		Ok(ini) => ini,
		Err(err) => {
			error!("{}", err);
			return;
		}
	};
	let conf = conf.general_section();

	debug!("file:    {}", matches.value_of("file").unwrap_or("None"));
	debug!("default: {}", conf.get("default-path").unwrap_or("None"));
	debug!("path:    {}", conf.get("path").unwrap_or("None"));
	debug!("date:    {}", conf.get("date").unwrap_or("None"));
	debug!("reset:   {}", matches.is_present("reset"));

	if let Some(src) = matches.value_of("file") {
		let src = Path::new(src);

		let dst = conf.get("path").expect("'path' is not defined");
		let mut dst = if dst.contains("{DATE}") {
			let date_format = conf.get("date").expect("'date' is not defined");
			let date = &Local::now().format(date_format).to_string();
			PathBuf::from(dst.replace("{DATE}", &date))
		} else {
			PathBuf::from(dst)
		};

		if dst.extension().is_none() {
			if let Some(ext) = src.extension() {
				dst.set_extension(ext);
			}
		}

		info!(
			"Moving '{}' to '{}'",
			&src.to_str().unwrap_or("???"),
			&dst.to_str().unwrap_or("???")
		);

		match move_file(&src, &dst) {
			Ok(()) => info!("File moved successfully!"),
			Err(err) => error!("Failed to move file: {}", err),
		}
	}
}
