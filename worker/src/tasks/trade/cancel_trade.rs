use mongodb::bson::doc;
use shared::{
	constant::{EVENT_TRADE_CANCELLED, TRADE_STATUS_CANCELED},
	models, BaseDocument, Trade,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi::{self},
	services,
	workers::{HandleParams, Task},
};

async fn on_trade_cancelled(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::TradeCanceled>()?;
	if let Some(ev) = event_parse {
		let trade_db = params.db.collection::<Trade>(models::trade::Trade::name().as_str());
		let query = doc! {
		  "trade_id": ev.trade,
		};
		let update = doc! {
		  "status": TRADE_STATUS_CANCELED,
		};
		let trade = trade_db.find_one(query.clone(), None).await?.unwrap();
		trade_db.update_one(query.clone(), update, None).await?;

		let mut nfts: Option<Vec<models::trade::Nft>> = None;
		if trade.nft.is_some() {
			nfts = Some(vec![trade.nft.unwrap()]);
		} else if trade.bundle.is_some() {
			nfts = trade.bundle;
		} else if trade.maybe_required.is_some() {
			nfts = trade.maybe_required;
		};

		let history = models::HistoryTx {
			trade_id: Some(ev.trade.to_string()),
			from: hex::encode(ev.who.0),
			id: None,
			amount: None,
			block_height: params.block.height,
			event: EVENT_TRADE_CANCELLED.to_string(),
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			nfts,
			pool: None,
			price: None,
			to: None,
			tx_hash: None,
			value: None,
			source: None,
			trade_type: Some(trade.trade_type),
		};
		services::history::upsert(history, params.db).await?;
	}
	Ok(())
}
pub fn tasks() -> Vec<Task> {
	vec![Task::new(EVENT_TRADE_CANCELLED, move |params| {
		Box::pin(on_trade_cancelled(params))
	})]
}
