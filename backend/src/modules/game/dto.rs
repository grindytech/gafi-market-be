use std::str::FromStr;

use crate::common::DBQuery;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::{models::game::Game, utils::vec_to_array};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct GameDTO {
	pub game_id: String,
	pub owner: String,
	pub id: Option<String>,

	pub is_verified: Option<bool>,
	pub twitter: Option<String>,
	pub website: Option<String>,

	pub discord: Option<String>,

	pub category: Option<String>,

	pub updated_at: Option<i64>,

	pub description: Option<String>,
	pub logo: Option<String>,
	pub banner: Option<String>,
	pub cover: Option<String>,

	pub name: Option<String>,
	pub collections: Option<Vec<String>>,
}

impl From<Game> for GameDTO {
	fn from(value: Game) -> Self {
		let decode_value = hex::decode(value.owner).expect("Failed to decode");
		GameDTO {
			id: Some(value.id.unwrap().to_string()),
			game_id: value.game_id,
			owner: subxt::utils::AccountId32(vec_to_array(decode_value)).to_string(),
			is_verified: value.is_verified,
			twitter: value.twitter,
			discord: value.discord,
			website: value.website,

			category: value.category,

			updated_at: Some(value.updated_at.timestamp_millis()),
			description: value.description,
			logo: value.logo,
			banner: value.banner,
			cover: value.cover,
			name: value.name,
			collections: value.collections,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryFindGame {
	pub game_id: Option<String>,
	pub owner: Option<String>,
	pub name: Option<String>,
	pub collection: Option<String>,
}

impl DBQuery for QueryFindGame {
	fn to_doc(&self) -> Document {
		let mut criteria = Document::new();
		if let Some(game_id) = &self.game_id {
			criteria.insert("game_id", game_id);
		}
		if let Some(owner) = &self.owner {
			let public_key = subxt::utils::AccountId32::from_str(&owner).expect("Failed to decode");
			criteria.insert("owner", hex::encode(public_key));
		}
		if let Some(name) = &self.name {
			criteria.insert(
				"name",
				doc! {
					 "$regex": name.to_string(),
					 "$options":"i"
				},
			);
		}
		if let Some(collection_id) = &self.collection {
			criteria.insert("collection_id", collection_id);
		}
		criteria
	}
}
