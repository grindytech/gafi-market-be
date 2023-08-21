use crate::modules::nft::dto::NFTDTO;
use serde::{Deserialize, Serialize};
use shared::bundle::Bundle;
use utoipa::ToSchema;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct BundleDTO {
	pub bundle_id: String,
	pub creator: String,
	pub name: String,
	pub description: String,
	pub items: Vec<NFTDTO>,
	pub market_type: String,
	pub status: String,
	pub price: i32,
	pub begin_at: i64,
	pub end_at: i64,
	pub update_at: i64,
	pub create_at: i64,
}
impl Into<Bundle> for BundleDTO {
	fn into(self) -> Bundle {
		Bundle {
			id: None,
			bundle_id: self.bundle_id,
			creator: self.creator,
			name: self.name,
			description: self.description,
			items: self.items.iter().map(|value| value.clone().into()).collect(),
			market_type: self.market_type,
			status: self.status,
			price: self.price,
			begin_at: self.begin_at,
			end_at: self.end_at,
			update_at: self.update_at,
			create_at: self.create_at,
		}
	}
}
impl From<Bundle> for BundleDTO {
	fn from(value: Bundle) -> Self {
		BundleDTO {
			bundle_id: value.bundle_id,
			creator: value.creator,
			name: value.name,
			description: value.description,
			items: value.items.iter().map(|value| value.clone().into()).collect(),
			market_type: value.market_type,
			status: value.status,
			price: value.price,
			begin_at: value.begin_at,
			end_at: value.end_at,
			update_at: value.update_at,
			create_at: value.create_at,
		}
	}
}

pub struct QueryFindBundles {
	pub bundle_id: Option<String>,
	pub creator: Option<String>,
	pub buyer: Option<String>,
	pub min_price: Option<i32>,
	pub max_price: Option<i32>,
	pub status: Option<String>,
	pub market_type: Option<String>,
}
