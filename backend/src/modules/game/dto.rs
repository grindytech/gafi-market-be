use crate::{common::DBQuery, modules::account::dto::SocialInfoDto};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::models::game::Game;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]

pub struct GameDTO {
	pub game_id: String,
	pub owner: String,
	pub is_verified: bool,
	pub social: SocialInfoDto,
	pub category: String,
	pub name: String,
	pub description: String,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
	pub create_at: i64,
}
impl From<Game> for GameDTO {
	fn from(value: Game) -> Self {
		GameDTO {
			game_id: value.game_id,
			owner: value.owner,
			is_verified: value.is_verified,
			social: value.social.into(),
			category: value.category,
			name: value.name,
			description: value.description,
			logo_url: value.logo_url,
			banner_url: value.banner_url,
			create_at: value.create_at,
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
		doc! {
			"$and": criteria
		}
	}
}
