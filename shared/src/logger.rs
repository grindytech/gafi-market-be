use log::{debug, error, info, warn};

use crate::constant::EMPTY_STR;
pub struct Logger {
	name: Option<String>,
}
impl Logger {
	pub fn new(name: Option<String>) -> Self {
		Self { name }
	}
	pub fn info(&self, msg: &str) {
		info!(
			"{} {}",
			self.name.clone().unwrap_or(EMPTY_STR.to_string()),
			msg
		)
	}
	pub fn warn(&self, msg: &str) {
		warn!(
			"{} {}",
			self.name.clone().unwrap_or(EMPTY_STR.to_string()),
			msg
		)
	}
	pub fn debug(&self, msg: &str) {
		debug!(
			"{} {}",
			self.name.clone().unwrap_or(EMPTY_STR.to_string()),
			msg
		)
	}
	pub fn error(&self, msg: &str) {
		error!(
			"{} {}",
			self.name.clone().unwrap_or(EMPTY_STR.to_string()),
			msg
		)
	}
}
