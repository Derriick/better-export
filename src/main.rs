use std::{
	error::Error,
	fs,
	path::Path,
};
use chrono::prelude::*;

use better_export::*;

fn main() -> Result<(), Box<dyn Error>> {
	let matches = create_app().get_matches();
	let conf = get_conf(&matches);
	let conf = conf.general_section();

	//println!("input file:   {}", matches.value_of("file").unwrap_or("no file"));
	//println!("path:         {}", conf.get("path").unwrap_or("None"));
	//println!("date:         {}", conf.get("date").unwrap_or("None"));
	//println!("path-default: {}", conf.get("path-default").unwrap_or("None"));
	//println!("reset:        {}", matches.is_present("reset"));

	if let Some(input_file) = matches.value_of("file") {
		let path = String::from(conf.get("path").unwrap());
		let path = if path.contains("{DATE}") {
			let date_format = conf.get("date").unwrap();
			path.replace("{DATE}", &Local::now().format(date_format).to_string())
		} else {
			path
		};
		let path = match Path::extension(Path::new(&path)) {
			Some(_) => path,
			None => match Path::extension(Path::new(input_file)) {
				Some(ext) => format!("{}.{}", path, ext.to_str().unwrap()),
				None => path,
			}
		};

		let _ = fs::rename(input_file, path)?;
	}

	Ok(())
}
