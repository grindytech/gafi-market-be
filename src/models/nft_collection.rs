use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NFTCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub collection_id: String,
    pub game_id: String, // Reference ID of game
    pub name: String,
    pub slug: String,
    pub category: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub minting_fee: String,
    pub is_verified: bool,
    pub update_at: i64,
    pub create_at: i64,
    pub raw: String,
}

pub const NAME: &str = "collection";
