use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Block {
	height: u64,
	hash: String,
	is_processed: bool,
}
pub const NAME: &str = "block";
