use mongodb::bson::{self, doc, Document};
use serde::{Deserialize, Serialize};
use shared::models::nft_collection::NFTCollection;
use utoipa::ToSchema;

use crate::{common::DBQuery, modules::nft::dto::NFTDTO};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct NFTCollectionDTO {
	pub id: Option<String>,
	pub collection_id: String,
	pub owner: String,
	pub slug: Option<String>,
	pub is_verified: Option<bool>,
	pub category: Option<String>,

	pub created_at: i64,

	pub updated_at: Option<i64>,
	pub games: Option<Vec<String>>,

	pub name: Option<String>,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
	pub external_url: Option<String>,
	pub nfts: Option<Vec<NFTDTO>>,
}
impl From<NFTCollection> for NFTCollectionDTO {
	fn from(value: NFTCollection) -> Self {
		let nfts: Option<Vec<NFTDTO>> =
			value.nfts.map(|nfts| nfts.iter().map(|nft| nft.clone().into()).collect());

		NFTCollectionDTO {
			collection_id: value.collection_id,
			slug: value.slug,
			name: value.name,
			is_verified: value.is_verified,
			category: value.category,
			owner: value.owner,
			created_at: value.created_at.timestamp_millis(),
			id: Some(value.id.unwrap().to_string()),
			updated_at: Some(value.updated_at.unwrap().timestamp_millis()),
			games: value.games,
			logo_url: value.logo_url,
			banner_url: value.banner_url,
			external_url: value.external_url,
			nfts,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindCollections {
	pub name: Option<String>,
	pub collection_id: Option<String>,
	pub owner: Option<String>,
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

		if let Some(owner) = &self.owner {
			criteria.push(doc! {
				"owner": owner
			});
		}
		if let Some(name) = &self.name {
			criteria.push(doc! {
				"name":{
					 "$regex": bson::Regex {
						pattern: name.to_string(),
						options: "i".to_string(),
					},
				}

			});
		}
		if let Some(game_id) = &self.game_id {
			criteria.push(doc! {
				"games": {
					"$in":[game_id]
				}
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
