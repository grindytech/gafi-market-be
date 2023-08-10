use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::account::SocialInfo;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct SocialDTO {
    pub twitter: Option<String>,
    pub web: Option<String>,
    pub medium: Option<String>,
    pub facebook: Option<String>,
    pub discord: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct GameDTO {
    pub game_id: String,
    pub owner: String,
    pub is_verified: bool,
    pub social: SocialDTO,
    pub category: String,
    pub name: String,
    pub description: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub create_at: i32,
}
