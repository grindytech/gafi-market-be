use std::collections::HashMap;

use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use shared::{
	models::nft::NFT,
	utils::{decimal128_to_string, string_decimal_to_number},
	Property,
};

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

	pub price: Option<String>,
}
impl Into<NFT> for NFTDTO {
	fn into(self) -> NFT {
		let config = shared::config::Config::init();
		NFT {
			token_id: self.token_id,
			id: None,
			collection_id: self.collection_id,
			is_burn: self.is_burn,
			status: self.status,
			price: None,
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
		let config = shared::config::Config::init();

		let price: Option<String> = match value.price {
			Some(v) => Some(decimal128_to_string(
				&v.to_string(),
				config.chain_decimal as i32,
			)),
			None => None,
		};

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
			created_by: subxt::utils::AccountId32(shared::utils::vec_to_array(
				hex::decode(value.created_by).expect("Failed to decode"),
			))
			.to_string(),
			attributes: Some(shared::utils::vec_property_to_hashmap(
				value.attributes.unwrap_or(vec![]),
			)),
			name: value.name,
			description: value.description,
			external_url: value.external_url,
			image: value.image,
			animation_url: value.animation_url,
			price,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NFTOwnerOfDto {
	pub id: Option<String>,
	pub token_id: String,
	pub collection_id: String,
	pub address: String,
	pub amount: u32,
	pub nft: NFTDTO,
}
impl From<shared::models::NFTOwner> for NFTOwnerOfDto {
	fn from(value: shared::models::NFTOwner) -> Self {
		let nft = value.nft.unwrap_or(vec![]).get(0).expect("nft not found").to_owned();
		Self {
			address: subxt::utils::AccountId32(shared::utils::vec_to_array(
				hex::decode(value.address).expect("Failed to decode"),
			))
			.to_string(),
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
	pub created_by: Option<String>,
	pub name: Option<String>,
	pub token_id: Option<String>,
	pub collection_id: Option<String>,
	pub attributes: Option<Vec<Property>>,
	pub price: Option<String>,
	pub onsale: Option<bool>,
}
impl DBQuery for QueryFindNFts {
	fn to_doc(&self) -> Document {
		let mut criteria = Document::new();
		if let Some(created_by) = &self.created_by {
			criteria.insert("created_by", created_by);
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
		if let Some(price) = &self.price {
			let config = shared::config::Config::init();
			let min_price = string_decimal_to_number(&price, config.chain_decimal as i32);
			let min_decimal: mongodb::bson::Decimal128 = min_price.parse().unwrap();

			criteria.insert(
				"price",
				doc! {
					"$gte":min_decimal
				},
			);
		}

		if let Some(onsale) = &self.onsale {
			criteria.insert(
				"price",
				doc! {
					"$exists":onsale

				},
			);
		}
		if let Some(attributes) = &self.attributes {
			let attr_value: Vec<Document> = attributes
				.into_iter()
				.map(|doc_v| {
					doc! {
						"attributes.key":{
							"$regex":doc_v.key.clone(),"$options":"i"
						},
					"attributes.value":
						{
							"$regex":doc_v.value.clone(),"$options":"i"
					}}
				})
				.collect();
			criteria.insert("$and", attr_value);
		}
		if let Some(collection_id) = &self.collection_id {
			criteria.insert("collection_id", collection_id);
		}
		criteria
	}
}
