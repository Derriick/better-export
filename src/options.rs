use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use std::{
	env,
	path::{Path, PathBuf},
};

macro_rules! default_path_conf {
	() => {
		"config.ini"
	};
}

mod key {
	pub const PATH_SRC: &str = "file";
	pub const PATH_DST: &str = "path";
	pub const PATH_DEFAULT: &str = "default-path";
	pub const FORMAT_DATE: &str = "date";
	pub const RESET: &str = "reset";
	pub const PATH_CONF: &str = "config";
	pub const VERBOSITY: &str = "verbose";

	pub mod short {
		pub const PATH_DST: &str = "p";
		pub const FORMAT_DATE: &str = "d";
		pub const RESET: &str = "r";
		pub const PATH_CONF: &str = "c";
		pub const VERBOSITY: &str = "v";
	}
}

mod default {
	pub const PATH_CONF: &str = default_path_conf!();
}

pub struct Options<'a>(ArgMatches<'a>);

impl Options<'_> {
	pub fn new() -> Self {
		let app = App::new(env!("CARGO_PKG_NAME"))
			.version(env!("CARGO_PKG_VERSION"))
			.author(env!("CARGO_PKG_AUTHORS"))
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.arg(Arg::with_name(key::PATH_SRC)
			     .value_name("FILE")
			     .help("Path of the file to move."))
			.arg(Arg::with_name(key::PATH_CONF)
			     .long(key::PATH_CONF)
			     .short(key::short::PATH_CONF)
			     .value_name("FILE")
			     .help(concat!("Set the path of the configuration file to use. Default is '", default_path_conf!(), "'."))
			     .takes_value(true))
			.arg(Arg::with_name(key::PATH_DST)
			     .long(key::PATH_DST)
			     .short(key::short::PATH_DST)
			     .value_name("FILE")
			     .help("Set the export path. {DATE} will be replaced by the current local date/time with the choosen format.\nIf no extension is specified, the extension of the input file will be used.")
			     .takes_value(true))
			.arg(Arg::with_name(key::FORMAT_DATE)
			     .long(key::FORMAT_DATE)
			     .short(key::short::FORMAT_DATE)
			     .value_name("FORMAT")
			     .help("Set the date format to use in the {DATE} placeholder.")
			     .takes_value(true))
			.arg(Arg::with_name(key::PATH_DEFAULT)
			     .long(key::PATH_DEFAULT)
			     .value_name("FILE")
			     .help("Set the default export path.")
			     .takes_value(true))
			.arg(Arg::with_name("reset")
			     .long(key::RESET)
			     .short(key::short::RESET)
			     .help("Reset the export path to the default export path."))
			.arg(Arg::with_name(key::VERBOSITY)
			     .long(key::VERBOSITY)
			     .short(key::short::VERBOSITY)
			     .multiple(true)
			     .help("Set the level of verbosity.\nThe number of occurences of this argument inscreases verbosity."));
		Self(app.get_matches())
	}

	pub fn path_src(&self) -> Option<&str> {
		self.0.value_of(key::PATH_SRC)
	}

	pub fn path_dst(&self) -> Option<&str> {
		self.0.value_of(key::PATH_DST)
	}

	pub fn path_default(&self) -> Option<&str> {
		self.0.value_of(key::PATH_DEFAULT)
	}

	pub fn format_date(&self) -> Option<&str> {
		self.0.value_of(key::FORMAT_DATE)
	}

	pub fn reset(&self) -> bool {
		self.0.is_present(key::RESET)
	}

	pub fn path_conf(&self) -> Result<PathBuf> {
		self.0
			.value_of(key::PATH_CONF)
			.map(PathBuf::from)
			.map(Ok)
			.unwrap_or_else(|| {
				let path = Path::new(default::PATH_CONF);
				if path.is_relative() {
					let path_exe = env::current_exe()?;
					let dir = path_exe
						.parent()
						.ok_or_else(|| anyhow!("Current exe file {:?} has no parent", path_exe))?;
					Ok(dir.join(path))
				} else {
					Ok(path.to_path_buf())
				}
			})
	}

	pub fn verbosity(&self) -> u64 {
		self.0.occurrences_of(key::VERBOSITY)
	}

	pub fn do_update_conf(&self) -> bool {
		self.path_dst().is_some()
			|| self.path_default().is_some()
			|| self.format_date().is_some()
			|| self.reset()
	}
}
