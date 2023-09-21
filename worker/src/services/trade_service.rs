use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
	Database,
};
use shared::{
	constant::{
		EVENT_AUCTION_CLAIMED, TRADE_SET_AUCTION, TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD,
	},
	history_tx, models, BaseDocument, Trade,
};
use subxt::utils::AccountId32;

use crate::{
	gafi::{
		self,
		runtime_types::{
			bounded_collections, gafi_support::game::types::Package,
			pallet_game::types::TradeConfig,
		},
	},
	workers::RpcClient,
};

use super::history_service;

pub async fn get_trade_config(
	trade_id: u32,
	api: &RpcClient,
) -> Result<
	TradeConfig<
		AccountId32,
		u128,
		bounded_collections::bounded_vec::BoundedVec<Package<u32, u32>>,
		u32,
	>,
	(),
> {
	let query_address = gafi::storage().game().trade_config_of(trade_id);
	let trade_config = api
		.storage()
		.at_latest()
		.await
		.expect("Fail to get blockchain storage")
		.fetch(&query_address)
		.await
		.expect("Fail to get trade_config")
		.expect("Fail to get trade_config");
	Ok(trade_config)
}

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

pub struct AuctionSetParams {
	pub source: Vec<models::trade::Nft>,
	pub maybe_price: Decimal128,
	pub owner: String,
	pub start_block: Option<u32>,
	pub duration: u32,
	pub trade_id: String,
}
pub async fn auction_set(
	params: AuctionSetParams,
	db: &Database,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
	let trade: Document = Trade {
		maybe_price: Some(params.maybe_price),
		start_block: params.start_block,
		duration: Some(params.duration),
		trade_id: params.trade_id.clone(),
		trade_type: TRADE_SET_AUCTION.to_string(),
		status: TRADE_STATUS_FOR_SALE.to_string(),
		source: Some(params.source),

		id: None,
		nft: None,
		maybe_required: None,
		bundle: None,
		wish_list: None,
		unit_price: None,
		price: None,
		owner: params.owner,
		end_block: None,
		sold: None,
	}
	.into();

	//create sale
	let trade_db = db.collection::<Trade>(&Trade::name());
	let options = UpdateOptions::builder().upsert(true).build();
	let query = doc! {
	  "trade_id": params.trade_id,
	};
	let upsert = doc! {
	  "$set": trade,
	};
	let rs = trade_db.update_one(query, upsert, options).await?;
	Ok(rs)
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
	  "status": status,
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
		nfts: params.nfts,
		pool: None,
		source: None,
		trade_id: Some(params.trade_id),
		trade_type: Some(params.trade_type),
		tx_hash: None,
		value: params.ask_price,
	};
	history_service::upsert(history, db).await?;
	Ok(())
}
