use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

use super::nft;

pub enum MarketType {
	OnSale,
	Canceled,
}
pub enum Status {
	OnSale,
	Sold,
	Canceled,
	Expired,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Items {
	pub token_id: String,
	pub game_id: String,
	pub quantity: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Bundle {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub bundle_id: String,
	pub creator: String,
	pub name: String,
	pub description: String,
	pub items: Vec<Items>,
	pub market_type: String,
	pub status: String,
	pub price: i32,
	pub begin_at: i64,
	pub end_at: i64,
	pub update_at: i64,
	pub create_at: i64,
}
impl BaseDocument for Bundle {
	fn name() -> String {
		"bundle".to_string()
	}
}

