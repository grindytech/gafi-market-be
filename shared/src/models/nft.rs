use mongodb::bson::{doc, oid::ObjectId, DateTime, Decimal128, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::BaseDocument;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct Property {
	pub key: String,
	pub value: String,
}
impl From<Document> for Property {
	fn from(doc: Document) -> Self {
		let key = doc.get_str("key").unwrap_or("");
		let value = doc.get_str("value").unwrap_or("");
		Self {
			key: key.to_string(),
			value: value.to_string(),
		}
	}
}
impl Into<Document> for Property {
	fn into(self) -> Document {
		let mut doc = Document::new();
		doc.insert("key", self.key);
		doc.insert("value", self.value);
		doc
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct NFT {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub token_id: String,
	pub collection_id: String,

	pub is_burn: Option<bool>,
	pub status: Option<String>,

	pub visitor_count: Option<i32>,
	pub favorite_count: Option<i32>,

	pub created_at: DateTime,
	pub updated_at: Option<DateTime>,
	pub supply: Option<u32>,

	pub created_by: String,

	pub attributes: Option<Vec<Property>>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub external_url: Option<String>,
	pub image: Option<String>,
	pub animation_url: Option<String>,
	// Min Price of NFT
	pub price: Option<Decimal128>,
}
impl BaseDocument for NFT {
	fn name() -> String {
		"nft".to_string()
	}
}
