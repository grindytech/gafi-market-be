use mongodb::bson::{doc, oid::ObjectId, Bson, Decimal128, Document};
use serde::{Deserialize, Serialize};
/* #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TypeEventTx {
	Mint,
	Transfer,
	Sale,
	Burn,
}
 */
use crate::BaseDocument;

pub type Nft = crate::models::trade::Nft;
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HistoryTx {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub tx_hash: Option<String>,

	pub extrinsic_index: i32,
	pub event_index: u32,
	pub block_height: u32,

	pub value: Option<Decimal128>,

	pub event: String,
	pub from: String,
	pub to: Option<String>,
	pub pool: Option<String>,
	pub nfts: Option<Vec<Nft>>,

	pub amount: Option<u32>,
	pub price: Option<Decimal128>,

	pub trade_id: Option<String>,
	pub trade_type: Option<String>,

	//swap
	pub source: Option<Vec<Nft>>,
}
impl Into<Document> for HistoryTx {
	fn into(self) -> Document {
		let source: Option<Vec<Document>> = match self.source {
			Some(nfts) => nfts
				.into_iter()
				.map(|nft| {
					let doc: Document = nft.into();
					Some(doc)
				})
				.collect(),
			None => None,
		};
		let nfts: Option<Vec<Document>> = match self.nfts {
			Some(nfts) => nfts
				.into_iter()
				.map(|nft| {
					let doc: Document = nft.into();
					Some(doc)
				})
				.collect(),
			None => None,
		};
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
			"nfts": Bson::from(nfts),

			"amount": self.amount,
			"price": self.price,

			"trade_id": self.trade_id,
			"source": source,
			"trade_type": self.trade_type,
		}
	}
}

impl BaseDocument for HistoryTx {
	fn name() -> String {
		"history_tx".to_string()
	}
}

//TODO separate history of nft, collection, game, pool
