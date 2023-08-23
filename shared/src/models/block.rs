use serde::{Deserialize, Serialize};

use crate::BaseDocument;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Block {
	pub height: u32,
	pub hash: String,
	// pub is_processed: bool,
}
impl BaseDocument for Block {
	fn name() -> String {
		"block".to_string()
	}
}

