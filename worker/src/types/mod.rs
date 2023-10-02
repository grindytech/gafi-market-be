use mongodb::bson::Decimal128;
use serde::Deserialize;
use shared::models;

pub struct AuctionClaimParams {
	pub trade_id: String,
	pub trade_type: String,
	pub from: String,
	pub to: Option<String>,
	pub price: Option<Decimal128>,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub nfts: Option<Vec<models::trade::Nft>>,
	pub ask_price: Option<Decimal128>,
}

pub struct AuctionSetParams {
	pub source: Vec<models::trade::Nft>,
	pub maybe_price: Decimal128,
	pub owner: String,
	pub start_block: Option<u32>,
	pub duration: u32,
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
}
pub struct SetPriceParams {
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub nft: models::trade::Nft,
	pub who: String,
	pub unit_price: Decimal128,
	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
}

pub struct ItemBoughtParams {
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub nft: models::trade::Nft,
	pub who: String,
	pub amount: u32,
	pub is_sold: bool,
}
