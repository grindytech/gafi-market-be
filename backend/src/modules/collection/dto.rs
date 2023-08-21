use serde::{Deserialize, Serialize};
use shared::models::nft_collection::NFTCollection;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct NFTCollectionDTO {
	pub collection_id: String,
	pub game_id: String,
	pub name: String,
	pub slug: String,
	pub logo_url: Option<String>,
	pub banner_url: Option<String>,
}
impl From<NFTCollection> for NFTCollectionDTO {
	fn from(value: NFTCollection) -> Self {
		NFTCollectionDTO {
			collection_id: value.collection_id,
			game_id: value.game_id,
			name: value.name,
			slug: value.slug,
			logo_url: value.logo_url,
			banner_url: value.banner_url,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryFindCollections {
	pub name: String,
	pub collection_id: String,
}
