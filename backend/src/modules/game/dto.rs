use std::collections::HashMap;

use crate::common::DBQuery;
use mongodb::bson::{doc, DateTime, Document};
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
	pub slug: Option<String>,

	pub metadata: Option<String>,
	pub attributes: Option<HashMap<String, String>>,

	/* #[schema(format = "date-time",value_type=Option<String> )]
	pub created_at: Option<i64>, */
	#[schema(format = "date-time",value_type=Option<String> )]
	pub updated_at: i64,
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
			slug: value.slug,
			/* 	created_at: Some(value.created_at.unwrap_or(value.updated_at).timestamp_millis()), */
			attributes: Some(shared::utils::vec_property_to_hashmap(
				value.attributes.unwrap_or(vec![]),
			)),
			metadata: value.metadata,
			updated_at: value.updated_at.timestamp_millis(),
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
				"name": name
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
