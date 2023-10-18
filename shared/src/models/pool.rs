use mongodb::bson::{doc, oid::ObjectId, Decimal128, Document};
use serde::{Deserialize, Serialize};

use crate::{BaseDocument, LootTable};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Pool {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub pool_id: String,
	pub owner: String,
	pub type_pool: String,

	pub mint_type: String,
	pub admin: String,

	pub minting_fee: Decimal128,
	pub begin_at: i64,
	pub end_at: i64,

	pub owner_deposit: String,

	pub updated_at: i64,
	pub created_at: i64,

	pub loot_table: Vec<LootTable>,
}
impl BaseDocument for Pool {
	fn name() -> String {
		"pool".to_string()
	}
}

impl Into<Document> for Pool {
	fn into(self) -> Document {
		let loot_table: Vec<Document> = self
			.loot_table
			.into_iter()
			.map(|t| {
				let doc = t.into();
				doc
			})
			.collect();
		doc! {
			"pool_id": self.pool_id,
			"owner": self.owner,
			"type_pool": self.type_pool,
			"loot_table": loot_table,
			"admin": self.admin,
			"mint_type":	self.mint_type,
			"minting_fee": self.minting_fee,
			"begin_at": self.begin_at,
			"end_at":  self.end_at,
			"owner_deposit": self.owner_deposit,
			"updated_at": self.updated_at,
			"created_at": self.created_at,
		}
	}
}
