//Data Transfer Object
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::nft::{Propertise, NFT};

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
impl From<NFT> for NftDTO {
    fn from(value: NFT) -> Self {
        NftDTO {
            token_id: value.token_id,
            collection_id: value.collection_id,
            amount: value.amount,
            is_burn: value.is_burn,
            name: value.name,
            description: value.description,
            status: value.status,
            external_url: value.external_url,
            weight: value.weight,
            img_url: value.img_url,
            visitor_count: value.visitor_count,
            favorite_count: value.favorite_count,
            propertise: value.propertise,
        }
    }
}
