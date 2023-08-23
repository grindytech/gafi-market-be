use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::models::nft_collection::NFTCollection;
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTCollectionDTO {
	pub collection_id: String,
	pub game_id: String,
	pub name: String,
	pub slug: String,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
}
impl From<NFTCollection> for NFTCollectionDTO {
	fn from(value: NFTCollection) -> Self {
		NFTCollectionDTO {
			collection_id: value.collection_id,
			game_id: value.game_id,
			name: value.name,
			slug: value.slug,
			logo_url: value.logo_url,
			banner_url: value.banner_url,
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
		doc! {
			"$and": criteria
		}
	}
}
