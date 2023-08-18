use crate::modules::nft::dto::NFTDTO;
use serde::{Deserialize, Serialize};
use shared::bundle::Bundle;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
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
}
