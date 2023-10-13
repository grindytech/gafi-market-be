use mongodb::bson::{doc, Decimal128, Document};
use serde::{Deserialize, Serialize};
use shared::history_tx::{self, HistoryTx};
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ToSchema)]
pub struct HistoryTxDTO {
	pub id: Option<String>,

	pub extrinsic_index: i32,
	pub event_index: u32,
	pub block_height: u32,

	pub value: Option<i64>,
	pub event: String,
	pub from: String,
	pub to: Option<String>,
	pub pool: Option<String>,

	pub nfts: Option<Vec<history_tx::Nft>>,

	pub amount: Option<u32>,
	/* pub price: String, */
	pub trade_id: Option<String>,
	pub trade_type: Option<String>,
	/* pub source: Option<Vec<history_tx::Nft>>, */
}
impl From<HistoryTx> for HistoryTxDTO {
	fn from(value: HistoryTx) -> Self {
		HistoryTxDTO {
			id: Some(value.id.unwrap().to_string()),

			value: None,

			event: value.event,
			from: value.from,
			to: value.to,
			block_height: value.block_height,
			event_index: value.event_index,
			extrinsic_index: value.extrinsic_index,
			pool: value.pool,
			nfts: value.nfts,
			amount: value.amount,

			trade_id: value.trade_id,
			trade_type: value.trade_type,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindTX {
	pub collection_id: Option<String>,
	pub token_id: Option<Vec<String>>,
	pub trade_id: Option<String>,
	pub trade_type: Option<String>,
	pub event: Option<String>,
	pub pool_id: Option<String>,
	pub address: Option<String>,
}
impl DBQuery for QueryFindTX {
	fn to_doc(&self) -> Document {
		let mut criteria: Vec<Document> = vec![];

		if let Some(collection_id) = &self.collection_id {
			criteria.push(doc! {
				"collection_id": collection_id
			});
		}
		if let Some(trade_id) = &self.trade_id {
			criteria.push(doc! {
				"trade_id": trade_id
			});
		}
		if let Some(trade_type) = &self.trade_type {
			criteria.push(doc! {
				"trade_type": trade_type
			});
		}
		if let Some(event) = &self.event {
			criteria.push(doc! {
				"event": event
			});
		}
		if let Some(address) = &self.address {
			criteria.push(doc! {
				"$or":[
					{"from": address},
					{"to":address}
				]

			});
		}
		if let Some(token_id) = &self.token_id {
			criteria.push(doc! {
				"nfts": {
					"$in":[token_id]
				}
			});
		}
		if let Some(pool_id) = &self.pool_id {
			criteria.push(doc! {
				"pool": pool_id
			});
		}

		if criteria.len() == 0 {
			doc! {}
		} else {
			doc! {
				"$and": criteria
			}
		}
	}
}
