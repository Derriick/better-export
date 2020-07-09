use clap::{App, Arg, ArgMatches};
use std::{
	env, io,
	path::{Path, PathBuf},
};

mod key {
	pub const PATH_SRC: &str     = "file";
	pub const PATH_DST: &str     = "path";
	pub const PATH_DEFAULT: &str = "default-path";
	pub const FORMAT_DATE: &str  = "date";
	pub const RESET: &str        = "reset";
	pub const PATH_CONF: &str    = "config";
	pub const VERBOSE: &str      = "verbose";
}

macro_rules! default_path_conf {
	() => {
		"config.ini"
	};
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
			     .short("c")
			     .long(key::PATH_CONF)
			     .value_name("FILE")
			     .help(concat!("Set the path of the configuration file to use. Default is '", default_path_conf!(), "'."))
			     .takes_value(true))
			.arg(Arg::with_name(key::PATH_DST)
			     .short("p")
			     .long(key::PATH_DST)
			     .value_name("FILE")
			     .help("Set the export path. {DATE} will be replaced by the current local date/time with the choosen format.\nIf no extension is specified, the extension of the input file will be used.")
			     .takes_value(true))
			.arg(Arg::with_name(key::FORMAT_DATE)
			     .short("d")
			     .long(key::FORMAT_DATE)
			     .value_name("FORMAT")
			     .help("Set the date format to use in the {DATE} placeholder.")
			     .takes_value(true))
			.arg(Arg::with_name("default-path")
			     .long("default-path")
			     .value_name("FILE")
			     .help("Set the default export path.")
			     .takes_value(true))
			.arg(Arg::with_name("reset")
			     .short("r")
			     .long("reset")
			     .help("Reset the export path to the default export path."))
			.arg(Arg::with_name(key::VERBOSE)
			     .short("v")
			     .multiple(true)
			     .help("Set the level of verbosity."));
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

	pub fn path_conf(&self) -> Result<PathBuf, io::Error> {
		match self.0.value_of(key::PATH_CONF) {
			Some(path) => Ok(PathBuf::from(path)),
			None => {
				let path = Path::new(default::PATH_CONF);
				if path.is_relative() {
					let path_exe = env::current_exe()?;
					let dir = match path_exe.parent() {
						Some(parent) => parent,
						None => return Err(io::Error::new(io::ErrorKind::Other, format!(
									"current exe file '{}' has no parent",
									path_exe.to_str().unwrap_or("???")
								))),
					};
					Ok(dir.join(path))
				} else {
					Ok(path.to_path_buf())
				}
			}
		}
	}

	pub fn verbose(&self) -> u64 {
		self.0.occurrences_of(key::VERBOSE)
	}
}
