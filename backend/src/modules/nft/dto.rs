use std::collections::HashMap;

use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use shared::models::nft::NFT;

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTDTO {
	pub id: Option<String>,
	pub token_id: String,
	pub collection_id: String,

	pub is_burn: Option<bool>,

	pub status: Option<String>,

	pub attributes: Option<HashMap<String, String>>,
	pub visitor_count: Option<i32>,
	pub favorite_count: Option<i32>,

	#[schema(format = "date-time",value_type=String )]
	pub created_at: i64,
	#[schema(format = "date-time",value_type=String )]
	pub updated_at: Option<i64>,

	pub created_by: String,
	pub supply: Option<u32>,

	pub name: Option<String>,
	pub description: Option<String>,
	pub external_url: Option<String>,
	pub image: Option<String>,
	pub animation_url: Option<String>,
}
impl Into<NFT> for NFTDTO {
	fn into(self) -> NFT {
		NFT {
			token_id: self.token_id,
			id: None,
			collection_id: self.collection_id,
			is_burn: self.is_burn,
			status: self.status,
			visitor_count: self.visitor_count,
			favorite_count: self.favorite_count,
			created_at: DateTime::from_millis(self.created_at),
			updated_at: Some(DateTime::from_millis(
				self.updated_at.unwrap_or(self.created_at),
			)),
			created_by: self.created_by,
			attributes: Some(shared::utils::hashmap_to_vec_property(
				self.attributes.unwrap_or(HashMap::new()),
			)),
			supply: self.supply,
			name: self.name,
			description: self.description,
			external_url: self.external_url,
			image: self.image,
			animation_url: self.animation_url,
		}
	}
}
impl From<NFT> for NFTDTO {
	fn from(value: NFT) -> Self {
		NFTDTO {
			id: Some(value.id.unwrap().to_string()),
			token_id: value.token_id,
			collection_id: value.collection_id,
			is_burn: value.is_burn,
			status: value.status,
			visitor_count: value.visitor_count,
			favorite_count: value.favorite_count,
			supply: value.supply,
			updated_at: Some(value.updated_at.unwrap_or(value.created_at).timestamp_millis()),
			created_at: value.created_at.timestamp_millis(),
			created_by: value.created_by,
			attributes: Some(shared::utils::vec_property_to_hashmap(
				value.attributes.unwrap_or(vec![]),
			)),
			name: value.name,
			description: value.description,
			external_url: value.external_url,
			image: value.image,
			animation_url: value.animation_url,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NFTOwnerOfDto {
	pub id: Option<String>,
	pub token_id: String,
	pub collection_id: String,
	pub address: String,
	pub amount: i32,
	pub nft: NFTDTO,
}
impl From<shared::models::NFTOwner> for NFTOwnerOfDto {
	fn from(value: shared::models::NFTOwner) -> Self {
		let nft = value.nft.unwrap_or(vec![]).get(0).expect("nft not found").to_owned();
		Self {
			address: value.address,
			amount: value.amount,
			collection_id: value.collection_id,
			id: Some(value.id.unwrap().to_string()),
			token_id: value.token_id,
			nft: NFTDTO::from(nft),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindNFts {
	pub address: Option<String>,
	pub name: Option<String>,
	pub token_id: Option<String>,
	pub collection_id: Option<String>,
}
impl DBQuery for QueryFindNFts {
	fn to_doc(&self) -> Document {
		let mut criteria = Document::new();
		if let Some(address) = &self.address {
			criteria.insert("address", address);
		}
		if let Some(name) = &self.name {
			criteria.insert(
				"name",
				mongodb::bson::Regex {
					pattern: name.to_string(),
					options: "i".to_string(),
				},
			);
		}
		if let Some(token_id) = &self.token_id {
			criteria.insert("token_id", token_id);
		}
		if let Some(collection_id) = &self.collection_id {
			criteria.insert("collection_id", collection_id);
		}
		criteria
	}
}
