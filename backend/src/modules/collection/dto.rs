use std::str::FromStr;

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::models::nft_collection::NFTCollection;
use utoipa::ToSchema;

use crate::common::DBQuery;

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
	pub logo: Option<String>,
	pub banner: Option<String>,
	pub cover: Option<String>,
	pub external_url: Option<String>,
	pub description: Option<String>,
	/* pub nfts: Option<Vec<NFTDTO>>, */
}
impl From<NFTCollection> for NFTCollectionDTO {
	fn from(value: NFTCollection) -> Self {
		/* 	let nfts: Option<Vec<NFTDTO>> =
		value.nfts.map(|nfts| nfts.iter().map(|nft| nft.clone().into()).collect()); */

		NFTCollectionDTO {
			collection_id: value.collection_id,
			slug: value.slug,
			name: value.name,
			description: value.description,
			is_verified: value.is_verified,
			category: value.category,
			owner: subxt::utils::AccountId32(shared::utils::vec_to_array(
				hex::decode(value.owner).expect("Failed to decode"),
			))
			.to_string(),
			created_at: value.created_at.timestamp_millis(),
			id: Some(value.id.unwrap().to_string()),
			updated_at: Some(value.updated_at.unwrap().timestamp_millis()),
			games: value.games,
			logo: value.logo,
			banner: value.banner,
			cover: value.cover,
			external_url: value.external_url,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindCollections {
	pub name: Option<String>,
	pub collection_id: Option<String>,
	pub owner: Option<String>,
	pub games: Option<Vec<String>>,
}
impl DBQuery for QueryFindCollections {
	fn to_doc(&self) -> Document {
		let mut criteria = Document::new();

		if let Some(collection_id) = &self.collection_id {
			criteria.insert("collection_id", collection_id);
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
		if let Some(game_id) = &self.games {
			criteria.insert(
				"games",
				doc! {
					"$all":game_id
				},
			);
		}
		criteria
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]

pub struct NFTCollectionSupplyDTO {
	pub total_supply: i32, //total supply data of collection
	pub owner: i32,        // Number owner of collection
}
impl NFTCollectionSupplyDTO {
	pub fn convert_document_to_dto(
		document: Document,
	) -> Result<NFTCollectionSupplyDTO, mongodb::error::Error> {
		let total_supply = document.get("total_supply").and_then(|value| match value {
			mongodb::bson::Bson::Int32(value) => Some(*value),
			_ => None,
		});
		let owner = document.get("owner").and_then(|value| match value {
			mongodb::bson::Bson::Int32(value) => Some(*value),
			_ => None,
		});
		Ok(NFTCollectionSupplyDTO {
			total_supply: total_supply.unwrap_or(0),
			owner: owner.unwrap_or(0),
		})
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NFTCollectionVolumeDTO {
	pub min_price: Option<String>,
	pub max_price: Option<String>,
	pub volume: Option<String>,
	pub sold: Option<String>,
}
impl NFTCollectionVolumeDTO {
	pub fn convert_document_to_dto(
		document: Document,
	) -> Result<NFTCollectionVolumeDTO, mongodb::error::Error> {
		let min_price = document.get("min_price").and_then(|value| match value {
			mongodb::bson::Bson::Decimal128(decimal) => Some(decimal.to_string()),
			_ => None,
		});

		let max_price = document.get("max_price").and_then(|value| match value {
			mongodb::bson::Bson::Decimal128(decimal) => Some(decimal.to_string()),
			_ => None,
		});

		let volume = document.get("volume").and_then(|value| match value {
			mongodb::bson::Bson::Decimal128(decimal) => Some(decimal.to_string()),
			_ => None,
		});

		let sold = document.get("sold").and_then(|value| match value {
			mongodb::bson::Bson::Int32(i) => Some(i.to_string()),
			_ => None,
		});

		Ok(NFTCollectionVolumeDTO {
			min_price,
			max_price,
			volume,
			sold,
		})
	}
}
