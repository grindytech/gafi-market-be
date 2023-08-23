use mongodb::bson::{doc, oid::ObjectId};
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
	pub tx_hash: String,
	pub status: String,
	pub error_message: String,
	pub value: i32,
	pub event: String,
	pub from: String,
	pub to: String,
	pub collection_id: String,
	pub game_id: String,
	pub token_id: String,
	pub raw: String,
}
impl BaseDocument for Transaction {
	fn name() -> String {
		"history_tx".to_string()
	}
}

