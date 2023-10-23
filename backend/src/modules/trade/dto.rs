use std::str::FromStr;

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::{history_tx::Nft, LootTableNft, Trade};
use utoipa::ToSchema;

use crate::common::DBQuery;
#[derive(Debug, Deserialize, Serialize, ToSchema)]

pub struct TradeDTO {
	pub trade_id: String,
	pub trade_type: String,
	pub owner: String,

	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
	pub duration: Option<u32>, //auct

	pub price: Option<String>,

	pub nft: Option<Nft>,                 //set buy, set price
	pub source: Option<Vec<Nft>>,         //swap, auction
	pub maybe_required: Option<Vec<Nft>>, //swap
	pub bundle: Option<Vec<Nft>>,         //bundle
	pub wish_list: Option<Vec<Nft>>,
	pub status: String,
	pub highest_bid: Option<String>,
}
impl From<Trade> for TradeDTO {
	fn from(value: Trade) -> Self {
		let config = shared::config::Config::init();
		let price: Option<String> = match value.price {
			Some(v) => Some(shared::utils::decimal128_to_string(
				&v.to_string(),
				config.chain_decimal as i32,
			)),
			None => None,
		};
		TradeDTO {
			trade_id: value.trade_id,
			trade_type: value.trade_type,
			owner: subxt::utils::AccountId32(shared::utils::vec_to_array(
				hex::decode(value.owner).expect("Failed to decode"),
			))
			.to_string(),
			start_block: value.start_block,
			end_block: value.end_block,
			duration: value.duration,
			price,
			nft: value.nft,
			source: value.source,
			maybe_required: value.maybe_required,
			bundle: value.bundle,
			wish_list: value.wish_list,
			status: value.status,
			highest_bid: match value.highest_bid {
				Some(v) => Some(v.to_string()),
				None => None,
			},
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindTrade {
	pub trade_id: Option<String>,
	pub trade_type: Option<String>,
	pub owner: Option<String>,
	pub price: Option<String>,
	pub bundle: Option<String>, // Search Bundle
	pub status: Option<String>,
	pub highest_bid: Option<String>,
	pub nft: Option<LootTableNft>,
}

impl DBQuery for QueryFindTrade {
	fn to_doc(&self) -> mongodb::bson::Document {
		let mut criteria = Document::new();
		if let Some(trade_id) = &self.trade_id {
			criteria.insert("trade_id", trade_id);
		}
		if let Some(trade_type) = &self.trade_type {
			criteria.insert("trade_type", trade_type);
		}
		if let Some(owner) = &self.owner {
			let public_key = subxt::utils::AccountId32::from_str(&owner).expect("Failed to decode");
			criteria.insert("owner", hex::encode(public_key));
		}
		if let Some(status) = &self.status {
			criteria.insert("status", status);
		}
		if let Some(nft) = &self.nft {
			criteria.insert(
				"$and",
				vec![doc! {
				"nft.item":nft.item.clone(),
				"nft.collection":nft.collection.clone()
				}],
			);
		}

		if let Some(bundle) = &self.bundle {
			criteria.insert("bundle", bundle);
		}
		criteria
	}
}
