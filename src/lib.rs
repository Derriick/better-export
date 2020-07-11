use anyhow::{anyhow, Result};
use log::{info, LevelFilter};
use std::{fs, path::Path};

mod config;
mod options;

use config::Config;
pub use options::Options;

pub fn run(opts: Options) -> Result<()> {
	let conf = Config::from(opts)?;

	//debug!("file:    {}", matches.value_of("file").unwrap_or("None"));
	//debug!("default: {}", conf.get("default-path").unwrap_or("None"));
	//debug!("path:    {}", conf.get("path").unwrap_or("None"));
	//debug!("date:    {}", conf.get("date").unwrap_or("None"));
	//debug!("reset:   {}", matches.is_present("reset"));

	if let Some(path_src) = conf.path_src() {
		let path_dst = conf.path_dst()?;
		info!("Moving {:?} to {:?}", path_src, path_dst);
		match move_file(&path_src, &path_dst) {
			Ok(()) => info!("File moved successfully!"),
			Err(err) => return Err(anyhow!("Failed to move file: {}", err)),
		};
	}

	Ok(())
}

pub fn level_filter(opts: &Options) -> LevelFilter {
	match opts.verbosity() {
		//0 => LevelFilter::Error, // error
		0 => LevelFilter::Warn,  // error + warn
		1 => LevelFilter::Info,  // error + warn + info
		2 => LevelFilter::Debug, // error + warn + info + debug
		_ => LevelFilter::Trace, // error + warn + info + debug + trace
	}
}

fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
	let from = Path::new(from.as_ref());
	if !from.exists() {
		Err(anyhow!("{:?} does not exist", from))
	} else if !from.is_file() {
		Err(anyhow!("{:?} is not a file", from))
	} else {
		match fs::rename(from, &to) {
			Ok(_) => Ok(()),
			Err(_) => {
				fs::copy(from, &to)?;
				fs::remove_file(from)?;
				Ok(())
			}
		}
	}
}
