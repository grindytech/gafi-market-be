use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{nft, BaseDocument};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
pub struct NFTOwner {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub token_id: String,
	pub collection_id: String,
	pub address: String,
	pub amount: i32,
	pub nft: Option<Vec<nft::NFT>>,
}
impl BaseDocument for NFTOwner {
	fn name() -> String {
		"nft_owner".to_string()
	}
}
