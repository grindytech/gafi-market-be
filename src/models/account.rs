use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct SocialInfo {
    pub twitter: Option<String>,
    pub web: Option<String>,
    pub medium: Option<String>,
    pub facebook: Option<String>,
    pub discord: Option<String>,
}
// Rename field id to be Serialize '_id => Working with DB
// Option => Indicate that value can be present Some(value) or absent None
// Can use [allow(non_snake_case)] marco if we want
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub address: String,
    pub balance: String,
    pub is_verified: bool,
    pub name: String,
    pub bio: String,
    pub social: SocialInfo,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub update_at: i64,
    pub create_at: i64,
}

pub const NAME: &str = "account";
