pub struct TradeDTO {
	pub id: Option<String>,

	pub trade_id: String,
	pub trade_type: String,
	pub owner: String,

	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
	pub duration: Option<u32>, //auct

	pub price: Option<String>,

	pub nft: Option<Nft>,                 //set buy, set price
	pub source: Option<Vec<Nft>>,         //swap, auction
	pub maybe_required: Option<Vec<Nft>>, //swap
	pub bundle: Option<Vec<Nft>>,         //bundle
	pub wish_list: Option<Vec<Nft>>,

	pub status: String,
	pub highest_bid: Option<String>,
}
