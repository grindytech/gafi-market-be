use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::history_tx::HistoryTx;
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct HistoryTxDTO {
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
impl From<HistoryTx> for HistoryTxDTO {
	fn from(value: HistoryTx) -> Self {
		HistoryTxDTO {
			tx_hash: value.tx_hash,
			status: value.status,
			error_message: value.error_message,
			value: value.value,
			event: value.event,
			from: value.from,
			to: value.to,
			collection_id: value.collection_id,
			game_id: value.game_id,
			token_id: value.token_id,
			raw: value.raw,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindHistory {
	pub tx_hash: Option<String>,
	pub status: Option<String>,
	pub from: Option<String>,
	pub to: Option<String>,
	pub game_id: Option<String>,
	pub collection_id: Option<String>,
	pub token_id: Option<String>,
}
impl DBQuery for QueryFindHistory {
	fn to_doc(&self) -> Document {
		let mut criteria: Vec<Document> = vec![];
		if let Some(tx_hash) = &self.tx_hash {
			criteria.push(doc! {
				"tx_hash": tx_hash
			});
		}
		if let Some(from) = &self.from {
			criteria.push(doc! {
				"from": from
			});
		}
		if let Some(to) = &self.to {
			criteria.push(doc! {
				"to": to
			});
		}
		if let Some(game_id) = &self.game_id {
			criteria.push(doc! {
				"game_id": game_id
			});
		}
		if let Some(status) = &self.status {
			criteria.push(doc! {
				"status": status
			});
		}
		if let Some(collection_id) = &self.collection_id {
			criteria.push(doc! {
				"collection_id": collection_id
			});
		}
		if let Some(token_id) = &self.token_id {
			criteria.push(doc! {
				"token_id": token_id
			});
		}
		doc! {
			"$and":criteria
		}
	}
}
