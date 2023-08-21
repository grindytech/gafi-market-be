use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;
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
	pub owner: ObjectId, //Reference to address of account
	pub type_pool: TypePool,
	pub minting_fee: String,
	pub begin_at: i64,
	pub end_at: i64,
	pub update_at: i64,
	pub create_at: i64,
}
impl BaseDocument for Pool {
	fn name() -> String {
		"pool".to_string()
	}
}

