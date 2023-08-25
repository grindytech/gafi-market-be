use mongodb::bson::{doc, oid::ObjectId, Bson, Document, Decimal128};
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
pub struct Nft {
	pub collection_id: String,
	pub token_id: String,
	pub amount: u32,
}
impl Into<Document> for Nft {
	fn into(self) -> Document {
		doc! {
			"collection_id": self.collection_id,
			"token_id": self.token_id,
			"amount": self.amount
		}
	}
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HistoryTx {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub tx_hash: Option<String>,

	pub extrinsic_index: u32,
	pub event_index: u32,
	pub block_height: u32,

	pub value: Decimal128,

	pub event: String,
	pub from: String,
	pub to: String,
	pub pool: Option<String>,
	pub nfts: Vec<Nft>,
}
impl Into<Document> for HistoryTx {
	fn into(self) -> Document {
		let nfts = self
			.nfts
			.into_iter()
			.map(|nft| {
				let doc: Document = nft.into();
				doc
			})
			.collect::<Vec<Document>>();
		doc! {
			"tx_hash": self.tx_hash,

			"extrinsic_index": self.extrinsic_index,
			"event_index": self.event_index,
			"block_height": self.block_height,

			"value": self.value,
			"event": self.event,
			"from": self.from,
			"to": self.to,
			"pool": self.pool,
			"nfts": Bson::from(nfts)
		}
	}
}

impl BaseDocument for HistoryTx {
	fn name() -> String {
		"history_tx".to_string()
	}
}
