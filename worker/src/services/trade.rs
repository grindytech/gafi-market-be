use mongodb::{bson::doc, Database};
use shared::{models, BaseDocument};
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
		.unwrap()
		.fetch(&query_address)
		.await
		.unwrap()
		.unwrap();
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
	let trade_config = api.storage().at_latest().await?.fetch(&query_address).await?.unwrap().0;
	Ok(trade_config)
}
