use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use crate::{BaseDocument, Property};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NFTCollection {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub collection_id: String,
	pub slug: Option<String>,
	pub category: Option<String>,
	pub is_verified: Option<bool>,
	pub updated_at: Option<DateTime>,
	pub created_at: DateTime,
	pub owner: String,
	pub games: Option<Vec<String>>,

	pub metadata: Option<String>,
	pub attributes: Option<Vec<Property>>,
}

impl BaseDocument for NFTCollection {
	fn name() -> String {
		"nft_collection".to_string()
	}
}

impl Into<Document> for NFTCollection {
	fn into(self) -> Document {
		let attributes = match self.attributes {
			Some(attr) => {
				let mut doc_vec: Vec<Document> = vec![];
				attr.into_iter().for_each(|property| {
					let doc = property.into();
					doc_vec.push(doc);
				});
				Some(doc_vec)
			},
			None => None,
		};
		doc! {
			"id": self.id,
			"collection_id": self.collection_id,
			"slug": self.slug,
			"category": self.category,
			"is_verified": self.is_verified,
			"updated_at": DateTime::now(),
			"created_at": self.created_at,
			"owner":self.owner,
			"games": self.games,
			"attributes": attributes,
		}
	}
}
