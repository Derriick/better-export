use chrono::prelude::*;
use env_logger;
use log::{debug, info, LevelFilter};
use std::{io, path::Path, path::PathBuf};

use better_export::*;

fn main() -> io::Result<()> {
	let matches = create_app().get_matches();

	let level_filter = match matches.occurrences_of("verbose") {
		//0 => LevelFilter::Error,
		0 => LevelFilter::Warn,
		1 => LevelFilter::Info,
		2 => LevelFilter::Debug,
		_ => LevelFilter::Trace,
	};
	env_logger::builder().filter_level(level_filter).init();

	let conf = get_conf(&matches)?;
	let conf = conf.general_section();

	debug!("file:    {}", matches.value_of("file").unwrap_or("None"));
	debug!("default: {}", conf.get("default-path").unwrap_or("None"));
	debug!("path:    {}", conf.get("path").unwrap_or("None"));
	debug!("date:    {}", conf.get("date").unwrap_or("None"));
	debug!("reset:   {}", matches.is_present("reset"));

	match matches.value_of("file") {
		Some(src) => {
			let src = Path::new(src);

			let dst = conf.get("path").expect("'path' is not defined");
			let mut dst = if dst.contains("{DATE}") {
				let date_format =
					conf.get("date").expect("'date' is not defined");
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
				"Move '{}' to '{}'",
				&src.to_str().unwrap_or("???"),
				&dst.to_str().unwrap_or("???")
			);
			move_file(&src, &dst)
		}
		None => Ok(()),
	}
}
