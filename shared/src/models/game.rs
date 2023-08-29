use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

use super::account::SocialInfo;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Game {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub game_id: String,
	pub owner: String, // Reffence to account address

	pub is_verified: Option<bool>,
	pub social: Option<SocialInfo>,
	pub category: Option<String>,
	pub name: Option<String>,
	pub slug: Option<String>,

	pub description: Option<String>,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,

	pub updated_at: Option<DateTime>,
	pub created_at: Option<DateTime>,
	pub collections: Option<Vec<String>>,
}

impl BaseDocument for Game {
	fn name() -> String {
		"game".to_string()
	}
}

impl Into<Document> for Game {
	fn into(self) -> Document {
		let social = match self.social {
			Some(s) => {
				let doc: Document = s.into();
				Some(doc)
			},
			None => None,
		};
		doc! {
			"id":self.id,
			"game_id":self.game_id,
			"owner":self.owner,
			"is_verified":self.is_verified,
			"social":social,
			"category":self.category,
			"name": self.name,
			"slug": self.slug,
			"description": self.description,
			"logo_url": self.logo_url,
			"banner_url": self.banner_url,
			"updated_at": DateTime::now(),
			"created_at": self.created_at,
			"collections": self.collections,
		}
	}
}
