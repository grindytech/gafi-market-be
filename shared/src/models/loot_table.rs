use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LootTableNft {
	pub collection: String,
	pub item: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LootTable {
	pub nft: Option<LootTableNft>,
	pub weight: u32,
}
