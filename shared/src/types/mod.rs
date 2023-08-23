use std::{error, fmt};

// Change the alias to `Box<error::Error>`.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync>>;

pub trait BaseDocument {
	fn name() -> String;
}
