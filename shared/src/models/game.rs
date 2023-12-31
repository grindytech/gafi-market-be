use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

use super::account::SocialInfo;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Game {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub game_id: String,
	pub owner: String,

	pub is_verified: Option<bool>,
	pub social: Option<SocialInfo>,
	pub category: Option<String>,

	pub description: Option<String>,
	pub logo: Option<String>,
	pub banner: Option<String>,
	pub cover: Option<String>,

	pub name: Option<String>,

	pub updated_at: DateTime,
	/* pub created_at: Option<DateTime>, */
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
			"updated_at": DateTime::now(),
			"collections": self.collections,
			"description": self.description,
			"logo": self.logo,
			"banner": self.banner,
			"cover": self.cover,
			"name": self.name,
		}
	}
}
