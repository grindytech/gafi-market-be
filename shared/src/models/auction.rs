use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Auction {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub auction_id: String,
	pub token_id: String,
	pub creator: String,
	pub method: String,
	pub status: String,
	pub reserve_price: i16,
	pub start_price: i32,
	pub end_price: i32,
	pub begin_at: i64,
	pub end_at: i64,
	pub update_at: i64,
	pub create_at: i64,
}
pub const NAME: &str = "auction";
