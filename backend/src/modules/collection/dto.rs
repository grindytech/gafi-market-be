use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use shared::models::nft_collection::NFTCollection;
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTCollectionDTO {
	pub collection_id: String,
	pub owner: String,

	pub name: Option<String>,
	pub slug: Option<String>,

	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
	pub is_verified: Option<bool>,
	pub category: Option<String>,

	pub external_url: Option<String>,
	pub created_at: DateTime,
}
impl From<NFTCollection> for NFTCollectionDTO {
	fn from(value: NFTCollection) -> Self {
		NFTCollectionDTO {
			collection_id: value.collection_id,
			name: value.name,
			slug: value.slug,
			logo_url: value.logo_url,
			banner_url: value.banner_url,
			is_verified: value.is_verified,
			category: value.category,
			external_url: value.external_url,
			owner: value.owner,
			created_at: value.created_at,
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
