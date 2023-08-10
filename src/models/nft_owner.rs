use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NFTOwner {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token_id: ObjectId,
    pub collection_id: ObjectId,
    pub address: ObjectId,
    pub amount: i32,
    pub lock: i32,
}
pub const NAME: &str = "nft_owner";
