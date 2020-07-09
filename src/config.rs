use chrono::prelude::*;
use ini::{ini::EscapePolicy, ini::Properties, Ini};
use log::error;
use std::{
	error::Error,
	path::PathBuf,
};
use crate::options::Options;

mod key {
	pub const PATH_DST: &str     = "path";
	pub const PATH_DEFAULT: &str = "default-path";
	pub const FORMAT_DATE: &str  = "date";
}

mod default {
	pub const PATH_DST: &str     = "file{DATE}";
	pub const PATH_DEFAULT: &str = PATH_DST;
	pub const FORMAT_DATE: &str  = "%Y%m%d_%H%M%S";
}

pub struct Config<'a> {
	ini: Ini,
	opts: Options<'a>,
}

impl<'a> Config<'a> {
	pub fn from(opts: Options<'a>) -> Result<Self, Box<dyn Error>> {
		let path_conf = opts.path_conf()?;
		let ini = match Ini::load_from_file_noescape(&path_conf) {
			Ok(mut ini) => match ini.section_mut(None::<String>) {
				Some(mut section) => {
					update_section(&mut section, &opts);
					ini
				}
				None => ini_from(&opts),
			}
			Err(_) => ini_from(&opts),
		};

		if let Err(err) = ini.write_to_file_policy(&path_conf, EscapePolicy::Nothing) {
			error!(
				"Failed to save configuration to file {}\n{}",
				path_conf.to_str().unwrap_or("???"),
				err
			);
		}

		Ok(Self { ini, opts })
	}

	pub fn path_src(&self) -> Option<PathBuf> {
		match self.opts.path_src() {
			Some(path) => Some(PathBuf::from(path)),
			None => None,
		}
	}

	pub fn path_dst(&self) -> Result<PathBuf, String> {
		let section = match self.ini.section(None::<String>) {
			Some(section) => section,
			None => return Err(String::from("General section not found in Ini")),
		};
		let path_dst = match section.get(key::PATH_DST) {
			Some(path) => path,
			None => return Err(format!("'{}' not found in general section not found of Ini", key::PATH_DST)),
		};
		let format_date = match section.get(key::FORMAT_DATE) {
			Some(format) => format,
			None => return Err(format!("'{}' not found in general section not found of Ini", key::FORMAT_DATE)),
		};
		let mut path_dst = if path_dst.contains("{DATE}") {
			let date = &Local::now().format(format_date).to_string();
			PathBuf::from(path_dst.replace("{DATE}", &date))
		} else {
			PathBuf::from(path_dst)
		};
		if path_dst.extension().is_none() {
			let path_src = match self.path_src() {
				Some(path) => path,
				None => return Err(String::from("Argument for source path not found in command")),
			};
			if let Some(ext) = path_src.extension() {
				path_dst.set_extension(ext);
			}
		}
		Ok(path_dst)
	}
}

fn ini_from(opts: &Options) -> Ini {
	let mut ini = Ini::new();
	ini.with_general_section()
		.set(key::PATH_DEFAULT, opts.path_default().unwrap_or(default::PATH_DEFAULT))
		.set(key::PATH_DST, opts.path_default().unwrap_or(default::PATH_DST))
		.set(key::FORMAT_DATE, opts.path_default().unwrap_or(default::FORMAT_DATE));
	ini
}

fn update_section(mut section: &mut Properties, opts: &Options) {
	update_value(&mut section, key::PATH_DEFAULT, opts.path_default());
	update_value(&mut section, key::FORMAT_DATE, opts.format_date());

	if opts.reset() {
		let clone = section.clone();
		section.insert(key::PATH_DST, clone.get(key::PATH_DEFAULT).unwrap());
	} else {
		update_value(&mut section, key::PATH_DST, opts.path_dst());
	}
}

fn update_value(section: &mut Properties, key: &str, value: Option<&str>) {
	if let Some(value) = value {
		section.insert(key, value);
	}
}
