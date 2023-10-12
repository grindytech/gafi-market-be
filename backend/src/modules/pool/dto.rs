use mongodb::bson::{doc, Decimal128, Document};
use serde::{Deserialize, Serialize};
use shared::{
	utils::{decimal_to_string, string_decimal_to_number},
	LootTable, Pool,
};
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
		PoolDTO {
			pool_id: value.pool_id,
			owner: value.owner,
			type_pool: value.type_pool,
			admin: value.admin,
			minting_fee: decimal_to_string(
				&value.minting_fee.to_string(),
				config.chain_decimal as i32,
			),

			begin_at: value.begin_at,
			end_at: value.end_at,
			owner_deposit: value.owner_deposit,
			updated_at: value.updated_at,
			created_at: value.created_at,
			loot_table: value.loot_table,
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
		let mut criteria: Vec<Document> = vec![];
		if let Some(pool_id) = &self.pool_id {
			criteria.push(doc! {
				"pool_id": pool_id
			});
		}
		if let Some(owner) = &self.owner {
			criteria.push(doc! {
				"owner": owner
			});
		}
		if let Some(type_pool) = &self.type_pool {
			criteria.push(doc! {
				"type_pool": type_pool
			});
		}
		if let Some(admin) = &self.admin {
			criteria.push(doc! {
				"admin": admin
			});
		}
		if let Some(owner_deposit) = &self.owner_deposit {
			criteria.push(doc! {
				"owner_deposit": owner_deposit
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
