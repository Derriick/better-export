use clap::{App, Arg, ArgMatches};
use ini::{Ini, ini::EscapePolicy, ini::Properties};

macro_rules! default_conf { () => ( "config.ini" )}

const DEFAULT_CONF: &str = default_conf!();
const DEFAULT_PATH: &str = "D:\\file_{DATE}";
const DEFAULT_DATE: &str = "%Y%m%d_%H%M%S";

pub fn create_app() -> App<'static, 'static> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(Arg::with_name("file")
			  .value_name("FILE")
			  .help("Path of the file to move."))
		.arg(Arg::with_name("config")
			  .short("c")
			  .long("config")
			  .value_name("PATH")
			  .help(concat!("Sets the path of the configuration file to use.\nDefault is ", default_conf!()))
			  .takes_value(true))
		.arg(Arg::with_name("path")
			  .short("p")
			  .long("path")
			  .value_name("PATH")
			  .help("Sets the export path. {DATE} will be replaced by the current local date/time with the choosen format.\nIf no extension is specified, the extension of the input file will be used.")
			  .takes_value(true))
		.arg(Arg::with_name("date")
			  .short("d")
			  .long("date")
			  .value_name("FORMAT")
			  .help("Sets the date format to use in the {DATE} placeholder.")
		     .takes_value(true))
		.arg(Arg::with_name("path-default")
			  .long("path-default")
			  .value_name("PATH")
			  .help("Sets the default export path.")
			  .takes_value(true))
		.arg(Arg::with_name("reset")
		     .long("reset")
		     .help("Reset the export path to its default"))
}

pub fn get_conf(matches: &ArgMatches) -> Ini {
	let path = matches.value_of("config").unwrap_or(DEFAULT_CONF);
	let mut conf = Ini::load_from_file_noescape(path).unwrap_or(Ini::new());
	update_ini(&mut conf, &matches);
	
	if let Err(err) = conf.write_to_file_policy(path, EscapePolicy::Nothing) {
		eprintln!("Failed to write configuration to file {}\n{}", path, err);
	}

	conf
}

fn update_ini(ini: &mut Ini, matches: &ArgMatches) {
	if ini.section(None::<String>).is_none() {
		ini.with_general_section()
			.set("path-default", DEFAULT_PATH)
			.set("path", DEFAULT_PATH)
			.set("date", DEFAULT_DATE);
	}
	let mut section = ini.general_section_mut();

	update_value(&mut section, "path-default", matches);
	update_value(&mut section, "date", matches);

	if matches.is_present("reset") {
		let path = section.clone();
		let path = path.get("path-default").unwrap();
		section.insert("path", path);
	} else {
		update_value(&mut section, "path", matches);
	}
}

fn update_value(section: &mut Properties, key: &str, matches: &ArgMatches) {
	if let Some(value) = matches.value_of(key) {
		section.insert(key, value);
	}
}
