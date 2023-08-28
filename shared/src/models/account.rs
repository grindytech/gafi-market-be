use mongodb::bson::{doc, oid::ObjectId, Bson, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::BaseDocument;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct SocialInfo {
	pub twitter: Option<String>,
	pub web: Option<String>,
	pub medium: Option<String>,
	pub facebook: Option<String>,
	pub discord: Option<String>,
}

impl Into<Document> for SocialInfo {
	fn into(self) -> Document {
		doc! {
			"twitter":self.twitter,
			"web": self.web,
			"medium": self.medium,
			"facebook": self.facebook,
			"discord": self.discord,
		}
	}
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Favorites {
	pub token_id: String,
	pub collection_id: String,
	pub amount: i32,
}
impl Into<Bson> for Favorites {
	fn into(self) -> Bson {
		// Convert your Favorites struct to a Bson document
		let mut doc = Document::new();
		doc.insert("token_id", self.token_id);
		doc.insert("collection_id", self.collection_id);
		doc.insert("amount", self.amount);
		Bson::Document(doc)
	}
}
// Can use [allow(non_snake_case)] marco if we want
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Account {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub address: String,
	pub balance: String,
	pub is_verified: bool,
	pub name: String,
	pub bio: String,
	pub social: SocialInfo,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
	pub favorites: Option<Vec<Favorites>>,
	pub update_at: i64,
	pub create_at: i64,
}
impl BaseDocument for Account {
	fn name() -> String {
		"account".to_string()
	}
}
