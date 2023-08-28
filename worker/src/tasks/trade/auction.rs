use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
};
use shared::{
	constant::{
		EVENT_AUCTION_CLAIMED, EVENT_SET_AUCTION, TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD,
	},
	history_tx, models, BaseDocument, Trade,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi, services,
	workers::{HandleParams, Task},
};

async fn on_auction_claimed(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::AuctionClaimed>()?;
	if let Some(ev) = event_parse {
		let trade_db = params.db.collection::<Trade>(Trade::name().as_str());
		let query = doc! {
		  "trade_id": ev.trade,
		};
		let trade = trade_db.find_one(query.clone(), None).await?.unwrap();
		let update = doc! {
		  "status": TRADE_STATUS_SOLD,
		};
		let config = shared::config::Config::init();
		trade_db.update_one(query.clone(), update, None).await?;
		let mut who = None;
		let mut to = None;
		let mut price = None;
		match ev.maybe_bid {
			Some((account, p)) => {
				who = Some(account.clone());
				price = Some(p);
				to = Some(hex::encode(account.0));
			},
			None => {},
		};
		let history = history_tx::HistoryTx {
			id: None,
			amount: None,
			price: trade.price,
			block_height: params.block.height,
			event: EVENT_AUCTION_CLAIMED.to_string(),
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			from: trade.owner.clone(),
			to,
			nfts: trade.bundle.clone(),
			pool: None,
			source: None,
			trade_id: Some(trade.trade_id),
			trade_type: Some(trade.trade_type),
			tx_hash: None,
			value: Some(
				shared::utils::string_decimal_to_number(
					&price.unwrap_or(0u128).to_string(),
					config.chain_decimal as i32,
				)
				.parse()?,
			),
		};
		services::history::upsert(history, params.db).await?;
		for nft in trade.source.unwrap() {
			if who.is_some() {
				services::nft::refresh_balance(
					who.clone().unwrap(),
					nft.collection.to_string(),
					nft.item.to_string(),
					params.db,
					params.api,
				)
				.await?;
			};
			let owner_u8 = shared::utils::vec_to_array(hex::decode(trade.owner.clone())?);
			services::nft::refresh_balance(
				subxt::utils::AccountId32::from(owner_u8),
				nft.collection.to_string(),
				nft.item.to_string(),
				params.db,
				params.api,
			)
			.await?;
		}
	}
	Ok(())
}
async fn on_auction_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::AuctionSet>()?;
	if let Some(ev) = event_parse {
		let maybe_price = match ev.maybe_price {
			Some(p) => {
				let config = shared::config::Config::init();
				let price = shared::utils::string_decimal_to_number(
					&p.to_string(),
					config.chain_decimal as i32,
				);
				Some(price)
			},
			None => None,
		};
		let maybe_price_decimal: Decimal128 = maybe_price.unwrap_or("0".to_string()).parse()?;
		let mut source: Vec<models::trade::Nft> = vec![];
		for nft in ev.source {
			source.push(models::trade::Nft {
				collection: nft.collection,
				item: nft.item,
				amount: nft.amount,
			});
		}
		let trade: Document = Trade {
			id: None,
			nft: None,
			maybe_required: None,
			source: Some(source.clone()),
			bundle: None,
			wish_list: None,

			maybe_price: Some(maybe_price_decimal),
			unit_price: None,
			price: None,

			owner: hex::encode(ev.who.0),

			start_block: ev.start_block,
			end_block: None,
			duration: Some(ev.duration),

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_AUCTION.to_string(),

			sold: None,
			status: TRADE_STATUS_FOR_SALE.to_string(),
		}
		.into();
		//create sale
		let trade_db = params.db.collection::<Trade>(&Trade::name());
		let options = UpdateOptions::builder().upsert(true).build();
		let query = doc! {
		  "trade_id": ev.trade.to_string(),
		};
		let upsert = doc! {
		  "$set": trade,
		};
		trade_db.update_one(query, upsert, options).await?;
		//refetch balance
		for nft in source {
			services::nft::refresh_balance(
				ev.who.clone(),
				nft.collection.to_string(),
				nft.item.to_string(),
				params.db,
				params.api,
			)
			.await?;
		}
	}
	Ok(())
}

pub fn tasks() -> Vec<Task> {
	vec![
		Task::new(EVENT_SET_AUCTION, move |params| {
			Box::pin(on_auction_set(params))
		}),
		Task::new(EVENT_AUCTION_CLAIMED, move |params| {
			Box::pin(on_auction_claimed(params))
		}),
	]
}
