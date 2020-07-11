use env_logger;
use log::error;

use better_export::*;

fn main() {
	let opts = Options::new();
	env_logger::builder()
		.filter_level(level_filter(&opts))
		.init();

	if let Err(err) = run(opts) {
		error!("{}", err);
	}
}
