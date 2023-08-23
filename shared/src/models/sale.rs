use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::BaseDocument;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TypeSale {
	FixPrice(bool),
	TimeAuction(bool),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Sale {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub sale_id: String,
	pub token_id: String,
	pub quantity: i16,
	pub creator: String,
	pub type_sale: TypeSale,
	pub method: String,
	pub list_price: i32,
	pub begin_at: i64,
	pub end_at: i64,
	pub update_at: i64,
	pub create_at: i64,
}
impl BaseDocument for Sale {
	fn name() -> String {
		"sale".to_string()
	}
}

