use mongodb::bson::{doc, oid::ObjectId, Decimal128, Document};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

/* pub enum TradeType {
	SetPrice,
	Swap,
	SetBuy,
	Wishlist,
	Bundle,
	Auction,
} */

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Nft {
	pub collection: u32,
	pub item: u32,
	pub amount: u32,
}

impl Into<Document> for Nft {
	fn into(self) -> Document {
		doc! {
			"collection": self.collection,
			"item": self.item,
			"amount": self.amount
		}
	}
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Trade {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub trade_id: String,
	pub trade_type: String,
	pub owner: String,

	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
	pub duration: Option<u32>, //auction

	pub price: Option<Decimal128>,

	pub nft: Option<Nft>,                 //set buy, set price
	pub source: Option<Vec<Nft>>,         //swap, auction
	pub maybe_required: Option<Vec<Nft>>, //swap
	pub bundle: Option<Vec<Nft>>,         //bundle
	pub wish_list: Option<Vec<Nft>>,
	pub status: String,                  //ForSale, Sold, Canceled, Expired
	pub highest_bid: Option<Decimal128>, //auction
}

impl Into<Document> for Trade {
	fn into(self) -> Document {
		let maybe_required: Option<Vec<Document>> = match self.maybe_required {
			Some(nfts) => nfts
				.into_iter()
				.map(|nft| {
					let doc: Document = nft.into();
					Some(doc)
				})
				.collect(),
			None => None,
		};
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
		let bundle: Option<Vec<Document>> = match self.bundle {
			Some(nfts) => nfts
				.into_iter()
				.map(|nft| {
					let doc: Document = nft.into();
					Some(doc)
				})
				.collect(),
			None => None,
		};
		let wish_list: Option<Vec<Document>> = match self.wish_list {
			Some(nfts) => nfts
				.into_iter()
				.map(|nft| {
					let doc: Document = nft.into();
					Some(doc)
				})
				.collect(),
			None => None,
		};
		let nft: Option<Document> = match self.nft {
			Some(nft) => {
				let doc: Document = nft.into();
				Some(doc)
			},
			None => None,
		};
		doc! {
			"trade_id": self.trade_id,
			"trade_type": self.trade_type,
			"owner": self.owner,

			"start_block": self.start_block,
			"end_block": self.end_block,
			"duration": self.duration,

			"price": self.price,

			"nft": nft,
			"maybe_required": maybe_required,
			"source": source,
			"bundle": bundle,
			"wish_list": wish_list,
			"status": self.status
		}
	}
}

impl BaseDocument for Trade {
	fn name() -> String {
		"trade".to_string()
	}
}
