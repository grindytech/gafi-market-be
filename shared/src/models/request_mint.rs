use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestMint {
	pub who: String,
	pub pool: String,
	pub target: String,
	pub block: u32,
	pub event_index: u32,
	pub execute_block: u32,
	pub extrinsic_index: i32,
}
impl Into<Document> for RequestMint {
	fn into(self) -> Document {
		doc! {
			"who":	self.who,
			"pool": self.pool,
			"target": self.target,
			"block_number": self.block,
			"event_index":	self.event_index,
			"execute_block": self.execute_block,
			"extrinsic_index": self.extrinsic_index,
		}
	}
}
impl BaseDocument for RequestMint {
	fn name() -> String {
		"request-mint".to_string()
	}
}
