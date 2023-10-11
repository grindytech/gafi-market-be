use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use shared::{
	constant::{
		EVENT_AUCTION_CLAIMED, EVENT_BID, EVENT_BOUGHT_ITEM, EVENT_SET_AUCTION, TRADE_BID_AUCTION,
		TRADE_SET_AUCTION, TRADE_SET_BUY, TRADE_SET_PRICE, TRADE_STATUS_FOR_SALE,
		TRADE_STATUS_SOLD,
	},
	history_tx, models, BaseDocument, HistoryTx, Trade,
};

use crate::{
	gafi::{self, runtime_types::gafi_support::game::types::Package},
	types::{
		AuctionBidParams, AuctionClaimParams, AuctionSetParams, ItemBoughtParams, SetPriceParams,
	},
	workers::RpcClient,
};

use super::history_service;

// pub async fn get_trade_config(
// 	trade_id: u32,
// 	api: &RpcClient,
// ) -> Result<
// 	TradeConfig<
// 		AccountId32,
// 		u128,
// 		bounded_collections::bounded_vec::BoundedVec<Package<u32, u32>>,
// 		u32,
// 	>,
// 	(),
// > {
// 	let query_address = gafi::storage().game().trade_config_of(trade_id);
// 	let trade_config = api
// 		.storage()
// 		.at_latest()
// 		.await
// 		.expect("Fail to get blockchain storage")
// 		.fetch(&query_address)
// 		.await
// 		.expect("Fail to get trade_config")
// 		.expect("Fail to get trade_config");
// 	Ok(trade_config)
// }

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
		highest_bid: None,
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
		tx_hash: None,
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

		tx_hash: None,
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
		event: TRADE_SET_PRICE.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		nfts: Some(vec![params.nft]),
		price: Some(params.unit_price),
		trade_id: Some(params.trade_id),

		pool: None,
		to: None,
		tx_hash: None,
		value: None,
		source: None,
		trade_type: None,
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
		event: TRADE_SET_BUY.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		nfts: Some(vec![params.nft]),
		trade_id: Some(params.trade_id),
		price: Some(params.unit_price),

		pool: None,
		to: None,
		tx_hash: None,
		value: None,
		source: None,
		trade_type: None,
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
	let config = shared::config::Config::init();
	let total_value: u128 =
		trade.price.ok_or("unit price parse u128 fail")?.to_string().parse::<u128>()?
			* u128::from(params.amount);
	let total_value_decimal: Decimal128 = shared::utils::string_decimal_to_number(
		&total_value.to_string(),
		config.chain_decimal as i32,
	)
	.parse()?;
	let history = models::HistoryTx {
		block_height: params.block_height,
		event: EVENT_BOUGHT_ITEM.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: trade.owner,
		to: Some(params.who),
		nfts: Some(vec![params.nft.clone()]),
		value: Some(total_value_decimal),
		amount: Some(params.amount),
		price: trade.price,
		trade_id: Some(trade.trade_id.clone()),
		id: None,
		pool: None,
		tx_hash: None,
		source: None,
		trade_type: None,
	};
	history_service::upsert(history, db).await?;
	if params.is_sold {
		update_trade_status(&trade.trade_id, TRADE_STATUS_SOLD, db).await?;
	}
	Ok(())
}

pub async fn refresh_highest_bid(
	trade_id: String,
	bid: Decimal128,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let trade_db = db.collection::<Trade>(&Trade::name());
	let query = doc! {
		"highest_bid": {
			"$lt": bid,
		},
		"trade_id": trade_id,
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
	let history = HistoryTx {
		block_height: params.block_height,
		event: EVENT_BID.to_string(),
		event_index: params.event_index,
		extrinsic_index: params.extrinsic_index,
		from: params.who,
		trade_id: Some(params.trade_id.clone()),
		price: Some(params.bid),
		trade_type: Some(TRADE_BID_AUCTION.to_string()),
		value: None,
		id: None,
		nfts: None,
		pool: None,
		source: None,
		to: None,
		tx_hash: None,
		amount: None,
	};
	history_service::upsert(history, db).await?;
	refresh_highest_bid(params.trade_id, params.bid, db).await?;
	Ok(())
}
