use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use shared::{
	models::account::{Account, SocialInfo},
	Favorites,
};

use crate::common::DBQuery;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]

pub struct AccountDTO {
	pub address: String,
	pub balance: String,
	pub is_verified: bool,
	pub name: String,
	pub bio: String,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
	pub updated_at: i64,
	pub created_at: i64,
	pub social: SocialInfoDto,
	pub favorites: Option<Vec<Favorites>>,
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
			updated_at: value.updated_at,
			created_at: value.created_at,
			social: value.social.into(),
			favorites: value.favorites,
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryFindAccount {
	pub address: Option<String>,
	pub name: Option<String>,
	pub favorites: Option<Vec<Favorites>>,
}
impl DBQuery for QueryFindAccount {
	fn to_doc(&self) -> Document {
		let mut criteria: Vec<Document> = vec![];
		if let Some(address) = &self.address {
			criteria.push(doc! {
				"address":address
			});
		}
		if let Some(name) = &self.name {
			criteria.push(doc! {
				"name":name
			});
		};
		doc! {
			"$and":criteria
		}
	}
}
