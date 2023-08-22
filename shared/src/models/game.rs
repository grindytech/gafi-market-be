use mongodb::bson::{doc, oid::ObjectId};
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
	pub update_at: i64,
	pub create_at: i64,
}

impl BaseDocument for Game {
	fn name() -> String {
		"game".to_string()
	}
}

