use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use shared::{
	constant::{
		EVENT_AUCTION_CLAIMED, EVENT_BID, EVENT_BOUGHT_ITEM, EVENT_BUNDLE_BOUGHT,
		EVENT_SET_AUCTION, EVENT_SET_BUNDLE, EVENT_SET_BUY, EVENT_SET_PRICE, EVENT_SET_SWAP,
		EVENT_SET_WISH_LIST, EVENT_SWAP_CLAIMED, EVENT_TRADE_CANCELLED, EVENT_WIST_LIST_FILLED,
		TRADE_SET_AUCTION, TRADE_SET_BUNDLE, TRADE_SET_BUY, TRADE_SET_PRICE, TRADE_SET_SWAP,
		TRADE_SET_WIST_LIST, TRADE_STATUS_CANCELED, TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD,
	},
	history_tx, models, BaseDocument, HistoryTx, Trade,
};

use crate::{
	gafi::{self, runtime_types::gafi_support::game::types::Package},
	types::{
		AuctionBidParams, AuctionClaimParams, AuctionSetParams, BundleBoughtParams,
		BundleSetParams, CancelTradeParams, ItemBoughtParams, SetPriceParams, SwapClaimedParams,
		SwapSetParams, WishlistFilledParams, WishlistSetParams,
	},
	workers::RpcClient,
};

use super::history_service;

pub async fn get_by_trade_id(
	db: &Database,
	trade_id: &str,
) -> Result<Option<models::Trade>, mongodb::error::Error> {
	let trade_db = db.collection::<models::Trade>(models::Trade::name().as_str());
	let trade = trade_db
		.find_one(
			doc! {
				"trade_id": trade_id
			},
			None,
		)
		.await;
	trade
}

pub async fn bundle_of(
	trade_id: u32,
	api: &RpcClient,
) -> shared::types::Result<Vec<Package<u32, u32>>> {
	let query_address = gafi::storage().game().bundle_of(trade_id);
	let trade_config = api
		.storage()
		.at_latest()
		.await?
		.fetch(&query_address)
		.await?
		.expect("Fail to get trade config")
		.0;
	Ok(trade_config)
}

pub async fn upsert_trade(
	trade: Trade,
	db: &Database,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
	let trade_db = db.collection::<Trade>(&Trade::name());
	let options = UpdateOptions::builder().upsert(true).build();
	let query = doc! {
	  "trade_id": trade.trade_id.clone(),
	};
	let trade_doc: Document = trade.into();
	let upsert = doc! {
	  "$set": trade_doc,
	};
	let rs = trade_db.update_one(query, upsert, options).await?;
	Ok(rs)
}

/// Set auction.
/// - create trade
/// - create history
pub async fn auction_set(params: AuctionSetParams, db: &Database) -> shared::Result<()> {
	let trade = Trade {
		price: Some(params.maybe_price),
		start_block: params.start_block,
		duration: Some(params.duration),
		trade_id: params.trade_id.clone(),
		trade_type: TRADE_SET_AUCTION.to_string(),
		status: TRADE_STATUS_FOR_SALE.to_string(),
		source: Some(params.source.clone()),
		owner: params.owner.clone(),

		id: None,
		nft: None,
		maybe_required: None,
		bundle: None,
		wish_list: None,
		end_block: None,
		highest_bid: Some("0".parse()?),
	};
	upsert_trade(trade, db).await?;
	let history = history_tx::HistoryTx {
		id: None,
		amount: None,
		price: Some(params.maybe_price),
		block_height: params.block_height,
		event: EVENT_SET_AUCTION.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.owner,
		source: Some(params.source),
		trade_id: Some(params.trade_id),
		trade_type: Some(TRADE_SET_AUCTION.to_string()),
		to: None,
		pool: None,

		value: None,
		nfts: None,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn update_trade_status(
	trade_id: &str,
	status: &str,
	db: &Database,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
	let trade_db = db.collection::<Trade>(Trade::name().as_str());
	let query = doc! {
	  "trade_id": trade_id,
	};
	let update = doc! {
	 "$set": { "status": status,}
	};
	let rs = trade_db.update_one(query.clone(), update, None).await?;
	Ok(rs)
}

pub async fn get_trade_by_trade_id(
	trade_id: &str,
	db: &Database,
) -> Result<Option<Trade>, mongodb::error::Error> {
	let trade_db = db.collection::<Trade>(Trade::name().as_str());
	let query = doc! {
	  "trade_id": trade_id,
	};
	let trade = trade_db.find_one(query.clone(), None).await?;
	Ok(trade)
}

/// Claim an auction.
/// - update trade status to SOLD
/// - create history
pub async fn auction_claim(params: AuctionClaimParams, db: &Database) -> shared::Result<()> {
	update_trade_status(&params.trade_id, TRADE_STATUS_SOLD, db).await?;
	let history = history_tx::HistoryTx {
		id: None,
		amount: None,
		price: params.price,
		block_height: params.block_height,
		event: EVENT_AUCTION_CLAIMED.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.from,
		to: params.to,
		source: params.nfts,
		value: params.ask_price,
		trade_id: Some(params.trade_id),
		trade_type: Some(params.trade_type),

		nfts: None,
		pool: None,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

/// On set price for a nft
/// - create trade
/// - create history
pub async fn set_price(params: SetPriceParams, db: &Database) -> shared::Result<()> {
	let trade = Trade {
		nft: Some(params.nft.clone()),
		trade_id: params.trade_id.clone(),
		owner: params.who.clone(),
		price: Some(params.unit_price),
		trade_type: TRADE_SET_PRICE.to_string(),
		status: TRADE_STATUS_FOR_SALE.to_string(),
		start_block: params.start_block,
		end_block: params.end_block,

		maybe_required: None,
		source: None,
		bundle: None,
		wish_list: None,
		duration: None,
		id: None,
		highest_bid: None,
	};
	let history = history_tx::HistoryTx {
		amount: None,
		block_height: params.block_height,
		event: EVENT_SET_PRICE.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		nfts: Some(vec![params.nft]),
		price: Some(params.unit_price),
		trade_id: Some(params.trade_id),

		pool: None,
		to: None,

		value: None,
		source: None,
		trade_type: Some(TRADE_SET_PRICE.to_string()),
		id: None,
	};

	upsert_trade(trade, db).await?;
	history_service::upsert(history, db).await?;

	Ok(())
}

/// On set price for a nft
/// - create trade
/// - create history
pub async fn set_buy(params: SetPriceParams, db: &Database) -> shared::Result<()> {
	let trade = Trade {
		nft: Some(params.nft.clone()),
		price: Some(params.unit_price),
		owner: params.who.clone(),
		start_block: params.start_block,
		end_block: params.end_block,
		trade_id: params.trade_id.clone(),
		trade_type: TRADE_SET_BUY.to_string(),
		status: TRADE_STATUS_FOR_SALE.to_string(),

		duration: None,
		id: None,
		maybe_required: None,
		source: None,
		bundle: None,
		wish_list: None,
		highest_bid: None,
	};
	let history = history_tx::HistoryTx {
		amount: None,
		block_height: params.block_height,
		event: EVENT_SET_BUY.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		nfts: Some(vec![params.nft]),
		trade_id: Some(params.trade_id),
		price: Some(params.unit_price),

		pool: None,
		to: None,

		value: None,
		source: None,
		trade_type: Some(TRADE_SET_BUY.to_string()),
		id: None,
	};

	upsert_trade(trade, db).await?;
	history_service::upsert(history, db).await?;

	Ok(())
}

/// On set price for a nft
/// - update trade status
/// - create history
pub async fn bought_item(params: ItemBoughtParams, db: &Database) -> shared::Result<()> {
	let trade = get_by_trade_id(db, &params.trade_id).await?.ok_or("trade not found")?;

	let from = match trade.trade_type.as_str() {
		TRADE_SET_PRICE => &trade.owner,
		TRADE_SET_BUY => &params.who,
		_ => "",
	};
	let to = match trade.trade_type.as_str() {
		TRADE_SET_PRICE => &params.who,
		TRADE_SET_BUY => &trade.owner,
		_ => "",
	};

	let history = models::HistoryTx {
		block_height: params.block_height,
		event: EVENT_BOUGHT_ITEM.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: from.to_string(),
		to: Some(to.to_string()),
		nfts: Some(vec![params.nft.clone()]),
		value: None,
		amount: Some(params.amount),
		price: trade.price,
		trade_id: Some(trade.trade_id.clone()),
		id: None,
		pool: None,

		source: None,
		trade_type: Some(trade.trade_type),
	};
	history_service::upsert(history, db).await?;
	if params.is_sold {
		update_trade_status(&trade.trade_id, TRADE_STATUS_SOLD, db).await?;
	}
	Ok(())
}

pub async fn update_highest_bid(
	trade_id: String,
	bid: Decimal128,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let trade_db = db.collection::<Trade>(&Trade::name());
	let query = doc! {
		"$and": [
			// {
			// 	"highest_bid": {
			// 		"$lt": Some(bid)
			// 	}
			// },
			{"trade_id": trade_id}
		]
	};
	let update = doc! {
		"$set":{
			"highest_bid": bid,
		}
	};
	let rs = trade_db.update_one(query, update, None).await?;
	Ok(rs)
}

pub async fn create_auction_bid(params: AuctionBidParams, db: &Database) -> shared::Result<()> {
	let trade = params.trade;
	let history = HistoryTx {
		block_height: params.block_height,
		event: EVENT_BID.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		trade_id: Some(trade.trade_id.clone()),
		price: Some(params.bid),
		trade_type: Some(trade.trade_type),
		value: None,
		id: None,
		nfts: None,
		pool: None,
		source: trade.source,
		to: None,
		tx_hash: None,
		amount: None,
	};
	history_service::upsert(history, db).await?;

	let bid: f64 = params.bid.to_string().parse()?;
	let highest_bid: f64 = trade.highest_bid.unwrap_or("0".parse()?).to_string().parse()?;
	if bid > highest_bid {
		update_highest_bid(trade.trade_id, params.bid, db).await?;
	}
	Ok(())
}

pub async fn cancel_trade(params: CancelTradeParams, db: &Database) -> shared::Result<()> {
	let trade = get_trade_by_trade_id(&params.trade_id, db).await?.ok_or("trade not found")?;
	update_trade_status(&params.trade_id, TRADE_STATUS_CANCELED, db).await?;
	let mut nfts: Option<Vec<models::trade::Nft>> = None;
	if trade.nft.is_some() {
		nfts = Some(vec![trade.nft.unwrap()]);
	} else if trade.bundle.is_some() {
		nfts = trade.bundle;
	} else if trade.maybe_required.is_some() {
		nfts = trade.maybe_required;
	};

	let history = models::HistoryTx {
		trade_id: Some(params.trade_id.clone()),
		from: params.who,
		id: None,
		amount: None,
		block_height: params.block_height,
		event: EVENT_TRADE_CANCELLED.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		nfts,
		pool: None,
		price: None,
		to: None,
		tx_hash: None,
		value: None,
		source: None,
		trade_type: Some(trade.trade_type),
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn set_swap(params: SwapSetParams, db: &Database) -> shared::Result<()> {
	let trade = Trade {
		id: None,
		nft: None,
		maybe_required: Some(params.required.clone()),
		source: Some(params.source.clone()),
		bundle: None,
		wish_list: None,

		price: params.price,

		owner: params.who.clone(),

		start_block: params.start_block,
		end_block: params.end_block,
		duration: None,

		trade_id: params.trade_id.clone(),
		trade_type: TRADE_SET_SWAP.to_string(),

		status: TRADE_STATUS_FOR_SALE.to_string(),
		highest_bid: None,
	};
	upsert_trade(trade, db).await?;
	let history = models::HistoryTx {
		trade_id: Some(params.trade_id),
		from: params.who,
		id: None,
		amount: None,
		block_height: params.block_height,
		event: EVENT_SET_SWAP.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		nfts: Some(params.required),
		pool: None,
		price: params.price,
		to: None,
		tx_hash: None,
		value: None,
		source: Some(params.source),
		trade_type: Some(TRADE_SET_SWAP.to_string()),
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn claim_swap(params: SwapClaimedParams, db: &Database) -> shared::Result<()> {
	update_trade_status(&params.trade.trade_id, TRADE_STATUS_SOLD, db).await?;
	let trade = params.trade;
	let history = models::HistoryTx {
		amount: None,
		block_height: params.block_height,
		event: EVENT_SWAP_CLAIMED.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: trade.owner.clone(),
		to: Some(params.who),
		id: None,
		nfts: trade.maybe_required.clone(),
		source: trade.source.clone(),
		pool: None,
		price: trade.price,
		trade_id: Some(trade.trade_id),
		trade_type: Some(trade.trade_type),
		tx_hash: None,
		value: None,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn set_wishlist(params: WishlistSetParams, db: &Database) -> shared::Result<()> {
	let trade = Trade {
		id: None,
		nft: None,
		maybe_required: None,
		source: None,
		bundle: None,
		wish_list: Some(params.wish_list),
		price: params.price,
		owner: params.who.clone(),
		start_block: params.start_block,
		end_block: params.end_block,
		duration: None,
		trade_id: params.trade_id.clone(),
		trade_type: TRADE_SET_WIST_LIST.to_string(),
		status: TRADE_STATUS_FOR_SALE.to_string(),
		highest_bid: None,
	};
	upsert_trade(trade.clone(), db).await?;
	let history = models::HistoryTx {
		amount: None,
		block_height: params.block_height,
		event: EVENT_SET_WISH_LIST.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: trade.owner.clone(),
		to: Some(params.who.clone()),
		id: None,
		nfts: trade.wish_list.clone(),
		source: None,
		pool: None,
		price: trade.price,
		trade_id: Some(params.trade_id),
		trade_type: Some(trade.trade_type),
		tx_hash: None,
		value: None,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn wishlist_filled(params: WishlistFilledParams, db: &Database) -> shared::Result<()> {
	update_trade_status(&params.trade.trade_id, TRADE_STATUS_SOLD, db).await?;
	let trade = params.trade;
	let history = history_tx::HistoryTx {
		id: None,
		amount: None,
		price: trade.price,
		block_height: params.block_height,
		event: EVENT_WIST_LIST_FILLED.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: trade.owner.clone(),
		to: Some(params.who),
		nfts: trade.wish_list.clone(),
		pool: None,
		source: None,
		trade_id: Some(trade.trade_id),
		trade_type: Some(trade.trade_type),
		tx_hash: None,
		value: None,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn set_bundle(params: BundleSetParams, db: &Database) -> shared::Result<()> {
	let trade = Trade {
		id: None,
		nft: None,
		maybe_required: None,
		source: None,
		bundle: Some(params.nfts.clone()),
		wish_list: None,
		price: params.price,
		owner: params.who.clone(),
		start_block: params.start_block,
		end_block: params.end_block,
		duration: None,
		trade_id: params.trade_id.clone(),
		trade_type: TRADE_SET_BUNDLE.to_string(),
		status: TRADE_STATUS_FOR_SALE.to_string(),
		highest_bid: None,
	};
	upsert_trade(trade, db).await?;
	let history = HistoryTx {
		amount: None,
		block_height: params.block_height,
		event: EVENT_SET_BUNDLE.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		id: None,
		nfts: Some(params.nfts),
		pool: None,
		price: params.price,
		source: None,
		to: None,
		trade_id: Some(params.trade_id),
		trade_type: Some(TRADE_SET_BUNDLE.to_string()),
		tx_hash: None,
		value: None,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}

pub async fn bundle_bought(params: BundleBoughtParams, db: &Database) -> shared::Result<()> {
	update_trade_status(&params.trade.trade_id, TRADE_STATUS_SOLD, db).await?;
	let trade = params.trade;
	let history = history_tx::HistoryTx {
		id: None,
		amount: None,
		price: trade.price,
		block_height: params.block_height,
		event: EVENT_BUNDLE_BOUGHT.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: trade.owner.clone(),
		to: Some(params.who),
		nfts: trade.bundle.clone(),
		pool: None,
		source: None,
		trade_id: Some(trade.trade_id),
		trade_type: Some(trade.trade_type),
		tx_hash: None,
		value: None,
	};
	history_service::upsert(history, db).await?;

	Ok(())
}
