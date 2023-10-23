use std::str::FromStr;

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use shared::{LootTable, Pool};
use utoipa::ToSchema;

use crate::common::DBQuery;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PoolDTO {
	pub pool_id: String,
	pub owner: String,
	pub type_pool: String,

	pub admin: String,

	pub minting_fee: String,

	pub start_block: i64,
	pub end_block: i64,

	pub owner_deposit: String,

	pub updated_at: i64,
	pub created_at: i64,
	/* 	#[schema(format = "Object", value_type = Object{

	})] */
	pub loot_table: Vec<LootTable>,
}
impl From<Pool> for PoolDTO {
	fn from(value: Pool) -> Self {
		let config = shared::config::Config::init();
		let minting_fee: String = shared::utils::decimal128_to_string(
			&value.minting_fee.to_string(),
			config.chain_decimal as i32,
		);

		PoolDTO {
			pool_id: value.pool_id,
			owner: subxt::utils::AccountId32(shared::utils::vec_to_array(
				hex::decode(value.owner).expect("Failed to decode"),
			))
			.to_string(),
			type_pool: value.type_pool,
			admin: value.admin,

			start_block: value.start_block,
			end_block: value.end_block,
			owner_deposit: value.owner_deposit,
			updated_at: value.updated_at,
			created_at: value.created_at,
			loot_table: value.loot_table,
			minting_fee,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindPool {
	pub pool_id: Option<String>,
	pub owner: Option<String>,
	pub type_pool: Option<String>,
	pub admin: Option<String>,
	pub owner_deposit: Option<String>,
}
impl DBQuery for QueryFindPool {
	fn to_doc(&self) -> mongodb::bson::Document {
		let mut criteria = Document::new();
		if let Some(pool_id) = &self.pool_id {
			criteria.insert("pool_id", pool_id);
		}
		if let Some(owner) = &self.owner {
			let public_key = subxt::utils::AccountId32::from_str(&owner).expect("Failed to decode");
			criteria.insert("owner", hex::encode(public_key));
		}
		if let Some(type_pool) = &self.type_pool {
			criteria.insert("type_pool", type_pool);
		}
		if let Some(admin) = &self.admin {
			criteria.insert("admin", admin);
		}
		if let Some(owner_deposit) = &self.owner_deposit {
			criteria.insert("owner_deposit", owner_deposit);
		}
		criteria
	}
}
