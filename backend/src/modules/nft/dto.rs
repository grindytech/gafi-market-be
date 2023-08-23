use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use shared::models::nft::{Propertise, NFT};

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct PropertiseDTO {
	pub key: String,
	pub value: String,
}
impl Into<Propertise> for PropertiseDTO {
	fn into(self) -> Propertise {
		Propertise {
			key: self.key,
			value: self.value,
		}
	}
}
impl From<Propertise> for PropertiseDTO {
	fn from(value: Propertise) -> Self {
		PropertiseDTO {
			key: value.key,
			value: value.value,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTDTO {
	pub token_id: String,
	pub collection_id: String,
	pub amount: i32,
	pub is_burn: bool,
	pub name: String,
	pub description: String,
	pub status: String,
	pub external_url: String,
	pub weight: String,
	pub img_url: String,
	pub visitor_count: i32,
	pub favorite_count: i32,
	pub create_at: i64,
	pub propertise: Vec<PropertiseDTO>,
	pub supply: Option<u32>,
}
impl Into<NFT> for NFTDTO {
	fn into(self) -> NFT {
		NFT {
			token_id: self.token_id,
			id: None,
			collection_id: self.collection_id,
			amount: self.amount,
			is_burn: self.is_burn,
			name: self.name,
			description: self.description,
			status: self.status,
			external_url: self.external_url,
			weight: self.weight,
			img_url: self.img_url,
			visitor_count: self.visitor_count,
			favorite_count: self.favorite_count,
			propertise: self.propertise.iter().map(|value| value.clone().into()).collect(),
			create_at: self.create_at,
			supply: self.supply
		}
	}
}
impl From<NFT> for NFTDTO {
	fn from(value: NFT) -> Self {
		NFTDTO {
			token_id: value.token_id,
			collection_id: value.collection_id,
			amount: value.amount,
			is_burn: value.is_burn,
			name: value.name,
			description: value.description,
			status: value.status,
			external_url: value.external_url,
			weight: value.weight,
			img_url: value.img_url,
			visitor_count: value.visitor_count,
			favorite_count: value.favorite_count,
			propertise: value.propertise.iter().map(|value| value.clone().into()).collect(),
			create_at: value.create_at,
			supply: value.supply
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
		let mut criteria: Vec<Document> = vec![];
		if let Some(name) = &self.name {
			criteria.push(doc! {
				"name": name
			});
		}
		if let Some(token_id) = &self.token_id {
			criteria.push(doc! {
				"token_id": token_id
			});
		}
		if let Some(collection_id) = &self.collection_id {
			criteria.push(doc! {
				"collection_id": collection_id
			});
		}
		doc! {
			"$and":criteria
		}
	}
}
