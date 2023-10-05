use std::collections::HashMap;

use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use shared::models::nft_collection::NFTCollection;
use utoipa::ToSchema;

use crate::{
	common::DBQuery,
	modules::{game::dto::GameDTO, nft::dto::NFTDTO},
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTCollectionDTO {
	pub id: Option<String>,
	pub collection_id: String,
	pub owner: String,
	pub slug: Option<String>,
	pub is_verified: Option<bool>,
	pub category: Option<String>,
	#[schema(format = "date-time",value_type=Option<String> )]
	pub created_at: Option<DateTime>,
	#[schema(format = "date-time",value_type=Option<String> )]
	pub updated_at: Option<DateTime>,
	pub games: Option<Vec<String>>,

	pub metadata: Option<String>,
	pub attributes: Option<HashMap<String, String>>,
}
impl From<NFTCollection> for NFTCollectionDTO {
	fn from(value: NFTCollection) -> Self {
		NFTCollectionDTO {
			collection_id: value.collection_id,
			slug: value.slug,
			is_verified: value.is_verified,
			category: value.category,
			owner: value.owner,
			created_at: Some(value.created_at),
			attributes: Some(shared::utils::vec_property_to_hashmap(
				value.attributes.unwrap_or(vec![]),
			)),
			games: value.games,
			id: Some(value.id.unwrap().to_string()),
			metadata: value.metadata,
			updated_at: value.updated_at,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindCollections {
	pub name: Option<String>,
	pub collection_id: Option<String>,
	pub game_id: Option<String>,
}
impl DBQuery for QueryFindCollections {
	fn to_doc(&self) -> Document {
		let mut criteria: Vec<Document> = vec![];

		if let Some(collection_id) = &self.collection_id {
			criteria.push(doc! {
				"collection_id": collection_id
			});
		}
		if let Some(name) = &self.name {
			criteria.push(doc! {
				"name": name
			});
		}
		if let Some(game_id) = &self.game_id {
			criteria.push(doc! {
				"game_id": game_id
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
