use serde::{Deserialize, Serialize};

use crate::{BaseDocument, Nft};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TradeType {
	SetPrice,
	Swap,
	SetBuy,
	Wishlist,
	Bundle,
	Auction,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Trade {
	pub trade_id: String,
	pub trade_type: TradeType,
	pub owner: String,
	pub maybe_price: Option<u32>,
	pub maybe_required: Option<Nft>,
	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
}
impl BaseDocument for Trade {
	fn name() -> String {
		"trade".to_string()
	}
}
