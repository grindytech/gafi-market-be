use mongodb::bson::Document;

use crate::common::DBQuery;

pub struct TradeDTO {
	pub id: Option<String>,

	pub trade_id: String,
	pub trade_type: String,
	pub owner: String,

	pub start_block: Option<u32>,
	pub end_block: Option<u32>,
	pub duration: Option<u32>, //auct

	pub price: Option<String>,

	pub nft: Option<shared::history_tx::Nft>, //set buy, set price
	pub source: Option<Vec<shared::history_tx::Nft>>, //swap, auction
	pub maybe_required: Option<Vec<shared::history_tx::Nft>>, //swap
	pub bundle: Option<Vec<shared::history_tx::Nft>>, //bundle
	pub wish_list: Option<Vec<shared::history_tx::Nft>>,
	pub status: String,
	pub highest_bid: Option<String>,
}

pub struct QueryFindTrade {
	pub trade_id: Option<String>,
	pub trade_type: Option<String>,
	pub owner: Option<String>,
	pub price: Option<String>,
	pub bundle: Option<String>, // Search Bundle
	pub status: Option<String>,
	pub highest_bid: Option<String>,
}

impl DBQuery for QueryFindTrade {
	fn to_doc(&self) -> mongodb::bson::Document {
		let mut criteria = Document::new();
		if let Some(trade_id) = &self.trade_id {
			criteria.insert("trade_id", trade_id);
		}
		if let Some(trade_type) = &self.trade_type {
			criteria.insert("trade_type", trade_type);
		}
		if let Some(owner) = &self.owner {
			criteria.insert("owner", owner);
		}
		if let Some(status) = &self.status {
			criteria.insert("status", status);
		}

		if let Some(bundle) = &self.bundle {
			criteria.insert("bundle", bundle);
		}
		criteria
	}
}
