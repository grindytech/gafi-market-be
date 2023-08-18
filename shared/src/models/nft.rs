use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Propertise {
	pub key: String,
	pub value: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NFT {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub token_id: String,
	pub collection_id: String,
	pub amount: i32,
	pub is_burn: bool,
	pub name: String,
	pub description: String,
	pub status: String,
	pub external_url: String,
	pub weight: String,
	pub img_url: String,
	pub visitor_count: i32,
	pub favorite_count: i32,
	pub propertise: Vec<Propertise>,
}
pub const NAME: &str = "nft";
