use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::account::SocialInfo;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct AccountDTO {
    pub address: String,
    pub balance: String,
    pub is_verified: bool,
    pub name: String,
    pub bio: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub update_at: i64,
    pub create_at: i64,
    pub social: SocialInfoDto,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct SocialInfoDto {
    pub twitter: Option<String>,
    pub web: Option<String>,
    pub medium: Option<String>,
    pub facebook: Option<String>,
    pub discord: Option<String>,
}

impl Into<SocialInfo> for SocialInfoDto {
    fn into(self) -> SocialInfo {
        SocialInfo {
            discord: self.discord,
            facebook: self.facebook,
            medium: self.medium,
            twitter: self.twitter,
            web: self.web,
        }
    }
}
