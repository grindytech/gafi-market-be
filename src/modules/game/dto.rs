use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{models::game::Game, modules::account::dto::SocialInfoDto};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct GameDTO {
    pub game_id: String,
    pub owner: String,
    pub is_verified: bool,
    pub social: SocialInfoDto,
    pub category: String,
    pub name: String,
    pub description: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub create_at: i64,
}
impl From<Game> for GameDTO {
    fn from(value: Game) -> Self {
        GameDTO {
            game_id: value.game_id,
            owner: value.owner,
            is_verified: value.is_verified,
            social: value.social.into(),
            category: value.category,
            name: value.name,
            description: value.description,
            logo_url: value.logo_url,
            banner_url: value.banner_url,
            create_at: value.create_at,
        }
    }
}
