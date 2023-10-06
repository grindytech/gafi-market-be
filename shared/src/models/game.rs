use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use crate::{BaseDocument, Property};

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
	pub slug: Option<String>,

	pub metadata: Option<String>,
	pub attributes: Option<Vec<Property>>,

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
			"id":self.id,
			"game_id":self.game_id,
			"owner":self.owner,
			"is_verified":self.is_verified,
			"social":social,
			"category":self.category,
			"slug": self.slug,
			"updated_at": DateTime::now(),
		/* 	"created_at": self.created_at, */
			"collections": self.collections,
			"attributes": attributes,
			"metadata": self.metadata,
		}
	}
}
