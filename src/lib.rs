use std::{fs, io, path::Path};

pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(
	from: P,
	to: Q,
) -> io::Result<()> {
	let from = Path::new(from.as_ref());
	if from.is_file() {
		match fs::rename(from, &to) {
			Ok(_) => Ok(()),
			Err(_) => {
				fs::copy(from, &to)?;
				fs::remove_file(from)
			}
		}
	} else {
		Err(io::Error::new(
			io::ErrorKind::Other,
			format!("'{}' is not a file", from.to_str().unwrap_or("???")),
		))
	}
}
