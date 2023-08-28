use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::{Nft, Trade, TradeType};
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct TradeDTO {
	pub trade_id: String,
	pub trade_type: TradeType,
	pub owner: String,
	pub maybe_price: Option<u32>,
	pub maybe_required: Option<Nft>,
	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
}

impl From<Trade> for TradeDTO {
	fn from(value: Trade) -> Self {
		TradeDTO {
			trade_id: value.trade_id,
			trade_type: value.trade_type,
			owner: value.owner,
			maybe_price: value.maybe_price,
			maybe_required: value.maybe_required,
			start_block: value.start_block,
			end_block: value.end_block,
		}
	}
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryFindTrade {
	pub trade_id: Option<String>,
	pub trade_type: TradeType,
	pub owner: Option<String>,
	pub maybe_price: Option<u32>,
}
impl DBQuery for QueryFindTrade {
	fn to_doc(&self) -> mongodb::bson::Document {
		let mut criteria: Vec<Document> = vec![];
		if let Some(trade_id) = &self.trade_id {
			criteria.push(doc! {
				"trade_id": trade_id
			});
		}
		if let Some(owner) = &self.owner {
			criteria.push(doc! {
				"owner": owner
			});
		}
		if let Some(maybe_price) = &self.maybe_price {
			criteria.push(doc! {
				"maybe_price": maybe_price
			});
		}

		/* 	criteria.push(doc! {
			"trade_type": TradeType::self.trade_type
		}); */

		doc! {
			"$and":criteria
		}
	}
}
