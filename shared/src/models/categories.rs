use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Categories {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub name: String,
	pub slug: String,
	pub created_at: DateTime,
}

impl Into<Document> for Categories {
	fn into(self) -> Document {
		doc! {
			"name":self.name,
			"slug":self.slug
		}
	}
}

impl BaseDocument for Categories {
	fn name() -> String {
		"categories".to_string()
	}
}
