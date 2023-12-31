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

	pub begin_at: i64,
	pub end_at: i64,

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
			owner: value.owner,
			type_pool: value.type_pool,
			admin: value.admin,

			begin_at: value.begin_at,
			end_at: value.end_at,
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
			criteria.insert("owner", owner);
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
