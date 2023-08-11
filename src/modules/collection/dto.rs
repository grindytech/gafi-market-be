use crate::modules::account::dto::SocialInfoDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct CollectionDTO {
    pub collection_id: String,
    pub game_id: String,
    pub name: String,
    pub slug: String,
    pub logo_url: String,
    pub banner_url: String,
    pub social: SocialInfoDto,
    
}
