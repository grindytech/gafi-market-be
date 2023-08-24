use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::history_tx::HistoryTx;
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct HistoryTxDTO {
	pub tx_hash: Option<String>,

	pub extrinsic_index: u32,
	pub event_index: u32,
	pub block_height: u32,

	pub status: Option<String>,
	pub value: u128,
	pub amount: u32,

	pub event: String,
	pub from: String,
	pub to: String,
	pub collection_id: String,
	pub token_id: String,
	pub pool: Option<String>,
}
impl From<HistoryTx> for HistoryTxDTO {
	fn from(value: HistoryTx) -> Self {
		HistoryTxDTO {
			tx_hash: value.tx_hash,
			status: value.status,
			value: value.value,
			event: value.event,
			from: value.from,
			to: value.to,
			collection_id: value.collection_id,
			token_id: value.token_id,
			amount: value.amount,
			block_height: value.block_height,
			event_index: value.event_index,
			extrinsic_index: value.extrinsic_index,
			pool: value.pool,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindTX {
	pub tx_hash: Option<String>,
	pub status: Option<String>,
	pub from: Option<String>,
	pub to: Option<String>,
	pub game_id: Option<String>,
	pub collection_id: Option<String>,
	pub token_id: Option<String>,
}
impl DBQuery for QueryFindTX {
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
