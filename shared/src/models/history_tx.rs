use mongodb::bson::{doc, oid::ObjectId, Bson, Document};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TypeEventTx {
	Mint,
	Transfer,
	Sale,
	Burn,
}

use crate::BaseDocument;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct HistoryTx {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub tx_hash: Option<String>,

	pub extrinsic_index: u32,
	pub event_index: u32,
	pub block_height: u32,

	pub status: Option<String>,
	// pub error_message: String,
	pub value: u128,
	pub amount: u32,

	pub event: String,
	pub from: String,
	pub to: String,
	pub collection_id: String,
	// pub game_id: String,
	pub token_id: String,
	// pub raw: String,
	pub pool: Option<String>,
}
impl Into<Document> for HistoryTx {
	fn into(self) -> Document {
		doc! {
			"tx_hash": self.tx_hash,

			"extrinsic_index": self.extrinsic_index,
			"event_index": self.event_index,
			"block_height": self.block_height,

			"status": self.status,
			"value": Bson::Decimal128(self.value.to_string().parse().ok().unwrap()),
			"event": self.event,
			"from": self.from,
			"to": self.to,
			"collection_id": self.collection_id,
			"token_id": self.token_id
		}
	}
}

impl BaseDocument for HistoryTx {
	fn name() -> String {
		"history_tx".to_string()
	}
}
