use mongodb::bson::Decimal128;
use shared::{models, Trade};

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

pub struct CancelTradeParams {
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
}

pub struct AuctionBidParams {
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
	pub bid: Decimal128,
	pub trade: Trade,
}

pub struct SwapSetParams {
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
	pub source: Vec<models::trade::Nft>,
	pub required: Vec<models::trade::Nft>,
	pub price: Option<Decimal128>,
	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
}

pub struct SwapClaimedParams {
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
	pub trade: Trade,
}

pub struct WishlistSetParams {
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
	pub wish_list: Vec<models::trade::Nft>,
	pub price: Option<Decimal128>,
	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
}

pub struct WishlistFilledParams {
	pub trade: Trade,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
}

pub struct BundleSetParams {
	pub trade_id: String,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
	pub nfts: Vec<models::trade::Nft>,
	pub price: Option<Decimal128>,
	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
}

pub struct BundleBoughtParams {
	pub trade: Trade,
	pub block_height: u32,
	pub event_index: u32,
	pub extrinsic_index: i32,
	pub who: String,
}
