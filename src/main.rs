use env_logger;
use log::{error, info, LevelFilter};

use better_export::*;

mod config;
mod options;

use config::Config;
use options::Options;

fn main() {
	let opts = Options::new();
	let level_filter = match opts.verbose() {
		//0 => LevelFilter::Error, // error
		0 => LevelFilter::Warn,  // error + warn
		1 => LevelFilter::Info,  // error + warn + info
		2 => LevelFilter::Debug, // error + warn + info + debug
		_ => LevelFilter::Trace, // error + warn + info + debug + trace
	};
	env_logger::builder().filter_level(level_filter).init();
	let conf = match Config::from(opts) {
		Ok(conf) => conf,
		Err(err) => {
			error!("{}", err);
			return;
		}
	};

	//debug!("file:    {}", matches.value_of("file").unwrap_or("None"));
	//debug!("default: {}", conf.get("default-path").unwrap_or("None"));
	//debug!("path:    {}", conf.get("path").unwrap_or("None"));
	//debug!("date:    {}", conf.get("date").unwrap_or("None"));
	//debug!("reset:   {}", matches.is_present("reset"));

	if let Some(path_src) = conf.path_src() {
		let path_dst = match conf.path_dst() {
			Ok(path) => path,
			Err(err) => {
				error!("{}", err);
				return;
			}
		};

		info!(
			"Moving '{}' to '{}'",
			&path_src.to_str().unwrap_or("???"),
			&path_dst.to_str().unwrap_or("???")
		);

		match move_file(&path_src, &path_dst) {
			Ok(()) => info!("File moved successfully!"),
			Err(err) => error!("Failed to move file: {}", err),
		};
	}
}
