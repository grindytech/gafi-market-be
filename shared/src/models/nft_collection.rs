use std::collections::HashMap;

use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NFTCollection {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub collection_id: String,
	pub name: Option<String>,
	pub slug: Option<String>,

	pub category: Option<String>,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
	pub is_verified: Option<bool>,
	pub updated_at: Option<DateTime>,
	pub created_at: DateTime,
	pub metadata: Option<String>,
	pub owner: String,
	pub external_url: Option<String>,
	pub games: Option<Vec<String>>,

	pub attributes: Option<HashMap<String, String>>,
}

impl BaseDocument for NFTCollection {
	fn name() -> String {
		"nft_collection".to_string()
	}
}

impl Into<Document> for NFTCollection {
	fn into(self) -> Document {
		doc! {
			"id": self.id,
			"collection_id": self.collection_id,
			"name": self.name,
			"slug": self.slug,
			"category": self.category,
			"logo_url": self.logo_url,
			"banner_url": self.banner_url,
			"is_verified": self.is_verified,
			"updated_at": DateTime::now(),
			"created_at": self.created_at,
			"owner":self.owner,
			"external_url": self.external_url,
			"games": self.games,
		}
	}
}
