use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

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

	pub is_burn: Option<bool>,

	pub name: Option<String>,
	pub description: Option<String>,
	pub status: Option<String>,

	pub external_url: Option<String>,
	pub weight: Option<String>,
	pub img_url: Option<String>,

	pub visitor_count: Option<i32>,
	pub favorite_count: Option<i32>,

	pub propertise: Option<Vec<Propertise>>,
	pub created_at: DateTime,
	pub supply: Option<u32>,
}
impl BaseDocument for NFT {
	fn name() -> String {
		"nft".to_string()
	}
}
