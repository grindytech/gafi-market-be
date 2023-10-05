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

	#[schema(format = "date-time",value_type=Option<String> )]
	pub created_at: Option<DateTime>,
	#[schema(format = "date-time",value_type=Option<String> )]
	pub updated_at: Option<DateTime>,
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
			created_at: value.created_at,
			attributes: Some(shared::utils::vec_property_to_hashmap(
				value.attributes.unwrap_or(vec![]),
			)),
			metadata: value.metadata,
			updated_at: value.updated_at,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryFindGame {
	pub game_id: Option<String>,
	pub owner: Option<String>,
	pub category: Option<String>,
	pub is_verified: Option<bool>,
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
		if let Some(category) = &self.category {
			criteria.push(doc! {
				"category": category
			});
		}
		if let Some(is_verified) = &self.is_verified {
			criteria.push(doc! {
				"is_verified": is_verified
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

/* #[derive(Deserialize, IntoParams)]
pub struct GameParams {
	pub search: String,
	pub page: u64,
	pub size: u64,
	pub order_by: String,
	pub desc: bool,
} */
