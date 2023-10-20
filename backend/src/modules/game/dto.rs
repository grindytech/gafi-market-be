use crate::common::DBQuery;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::{models::game::Game, SocialInfo};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct GameDTO {
	pub game_id: String,
	pub owner: String,
	pub id: Option<String>,

	pub is_verified: Option<bool>,
	pub social: Option<SocialInfo>,
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
		GameDTO {
			id: Some(value.id.unwrap().to_string()),
			game_id: value.game_id,
			owner: value.owner,
			is_verified: value.is_verified,
			social: match value.social {
				Some(s) => Some(s.into()),
				None => None,
			},
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
		let mut criteria: Vec<Document> = vec![];
		if let Some(game_id) = &self.game_id {
			criteria.push(doc! {
				"game_id": game_id
			});
		}
		if let Some(owner) = &self.owner {
			criteria.push(doc! {
				"owner": owner
			});
		}
		if let Some(name) = &self.name {
			criteria.push(doc! {
				"name":{
					 "$regex": name.to_string(),
					 "$options":"i"
				}
			});
		}
		if let Some(collection_id) = &self.collection {
			criteria.push(doc! {
				"collection_id": collection_id
			});
		}
		if criteria.len() == 0 {
			doc! {}
		} else {
			doc! {
				"$and": criteria
			}
		}
	}
}
