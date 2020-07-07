use chrono::prelude::*;
use std::{io, path::Path, path::PathBuf};

use better_export::*;

fn main() -> io::Result<()> {
	let matches = create_app().get_matches();
	let conf = get_conf(&matches)?;
	let conf = conf.general_section();

	println!("file:    {}", matches.value_of("file").unwrap_or("no file"));
	println!("default: {}", conf.get("path-default").unwrap_or("None"));
	println!("path:    {}", conf.get("path").unwrap_or("None"));
	println!("date:    {}", conf.get("date").unwrap_or("None"));
	println!("reset:   {}", matches.is_present("reset"));

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

			println!(
				"Move '{}' to '{}'",
				&src.to_str().unwrap_or("???"),
				&dst.to_str().unwrap_or("???")
			);
			move_file(&src, &dst)
		}
		None => Ok(()),
	}
}
