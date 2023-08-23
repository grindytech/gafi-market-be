use mongodb::bson::{doc, oid::ObjectId, Decimal128};
use serde::{Deserialize, Serialize};

use crate::{BaseDocument, LootTable};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TypePool {
	DynamicPool(String),
	StablePool(String),
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pool {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub pool_id: String,
	pub owner: String,
	pub type_pool: TypePool,

	pub mint_type: String,
	pub admin: String,

	pub minting_fee: String,
	pub begin_at: i64,
	pub end_at: i64,

	pub owner_deposit: String,

	pub update_at: i64,
	pub create_at: i64,

	pub loot_table: Vec<LootTable>,
}
impl BaseDocument for Pool {
	fn name() -> String {
		"pool".to_string()
	}
}
