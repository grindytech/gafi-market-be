//Data Transfer Object
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::nft::Propertise;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct PropertiseDTO {
    pub key: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NftDTO {
    pub token_id: String,
    pub collection_id: String,
    pub amount: i32,
    pub is_burn: bool,
    pub name: String,
    pub description: String,
    pub status: String,
    pub external_url: String,
    pub weight: String,
    pub img_url: String,
    pub visitor_count: i32,
    pub favorite_count: i32,
    pub propertise: Vec<Propertise>,
}
