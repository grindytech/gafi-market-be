use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::account::{Account, SocialInfo};

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
impl From<Account> for AccountDTO {
    fn from(value: Account) -> Self {
        AccountDTO {
            address: value.address,
            balance: value.balance,
            is_verified: value.is_verified,
            name: value.name,
            bio: value.bio,
            logo_url: value.logo_url,
            banner_url: value.banner_url,
            update_at: value.update_at,
            create_at: value.create_at,
            social: value.social.into(),
        }
    }
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

impl From<SocialInfo> for SocialInfoDto {
    fn from(value: SocialInfo) -> Self {
        SocialInfoDto {
            twitter: value.twitter,
            web: value.web,
            medium: value.medium,
            facebook: value.facebook,
            discord: value.discord,
        }
    }
}
