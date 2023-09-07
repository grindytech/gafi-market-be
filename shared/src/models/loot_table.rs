use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct LootTableNft {
	pub collection: String,
	pub item: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct LootTable {
	pub nft: Option<LootTableNft>,
	pub weight: u32,
}
impl Into<Document> for LootTableNft {
	fn into(self) -> Document {
		doc! {
			"collection": self.collection,
			"item": self.item
		}
	}
}
impl Into<Document> for LootTable {
	fn into(self) -> Document {
		let mut nft: Option<Document> = None;
		if let Some(n) = self.nft {
			nft = Some(n.into());
		}
		doc! {
			"nft": nft,
			"weight": self.weight
		}
	}
}
