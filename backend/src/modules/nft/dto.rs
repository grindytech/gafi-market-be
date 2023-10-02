use std::collections::HashMap;

use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use shared::models::nft::{Property, NFT};

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTDTO {
	pub token_id: String,
	pub collection_id: String,

	pub is_burn: Option<bool>,

	pub name: Option<String>,
	pub description: Option<String>,
	pub status: Option<String>,

	pub external_url: Option<String>,
	pub weight: Option<String>,
	pub img_url: Option<String>,
	pub metadata: Option<String>,
	pub attributes: Option<HashMap<String, String>>,

	pub visitor_count: Option<i32>,
	pub favorite_count: Option<i32>,

	pub propertise: Option<Vec<Property>>,

	#[schema(format = "date-time",value_type=String )]
	pub created_at: DateTime,
	#[schema(format = "date-time",value_type=String )]
	pub updated_at: Option<DateTime>,

	pub created_by: String,

	pub supply: Option<u32>,
}
impl Into<NFT> for NFTDTO {
	fn into(self) -> NFT {
		NFT {
			token_id: self.token_id,
			id: None,
			collection_id: self.collection_id,
			is_burn: self.is_burn,
			name: self.name,
			description: self.description,
			status: self.status,
			external_url: self.external_url,
			weight: self.weight,
			img_url: self.img_url,
			visitor_count: self.visitor_count,
			favorite_count: self.favorite_count,
			properties: self.propertise,
			created_at: self.created_at,
			supply: self.supply,
			updated_at: self.updated_at,
			created_by: self.created_by,
			metadata: self.metadata,
			attributes: self.attributes,
		}
	}
}
impl From<NFT> for NFTDTO {
	fn from(value: NFT) -> Self {
		NFTDTO {
			token_id: value.token_id,
			collection_id: value.collection_id,

			is_burn: value.is_burn,
			name: value.name,
			description: value.description,
			status: value.status,
			external_url: value.external_url,
			weight: value.weight,
			img_url: value.img_url,
			visitor_count: value.visitor_count,
			favorite_count: value.favorite_count,
			propertise: value.properties,
			created_at: value.created_at,
			supply: value.supply,
			metadata: value.metadata,
			updated_at: value.updated_at,
			created_by: value.created_by,
			attributes: value.attributes,
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
		if criteria.len() == 0 {
			doc! {}
		} else {
			doc! {
				"$and": criteria
			}
		}
	}
}
