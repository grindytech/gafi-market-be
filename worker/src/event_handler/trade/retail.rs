use mongodb::bson::Decimal128;
use shared::{
	constant::{EVENT_BOUGHT_ITEM, EVENT_SET_BUY, EVENT_SET_PRICE},
	models,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi,
	services::{self, trade_service},
	types::{ItemBoughtParams, SetPriceParams},
	workers::{EventHandle, HandleParams},
};

async fn on_item_bought(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemBought>()?;
	if let Some(ev) = event_parse {
		let trade = services::trade_service::get_by_trade_id(params.db, &ev.trade.to_string())
			.await?
			.unwrap();

		let bundle_of = trade_service::bundle_of(ev.trade, params.api).await?;
		let trade_item = bundle_of.get(0).unwrap();
		let is_sold = trade_item.amount == 0;
		let nft = trade.nft.unwrap();

		let extrinsic_index = params.extrinsic_index.unwrap();
		let bought_params = ItemBoughtParams {
			amount: ev.amount,
			block_height: params.block.height,
			event_index: params.ev.index(),
			extrinsic_index,
			is_sold,
			nft: nft.clone(),
			trade_id: ev.trade.to_string(),
			who: hex::encode(ev.who.0),
		};
		services::trade_service::bought_item(bought_params, params.db).await?;
		services::nft_service::refresh_balance(
			ev.who,
			nft.collection.to_string(),
			nft.item.to_string(),
			params.db,
			params.api,
		)
		.await?;
	}

	Ok(())
}

//game::BuySet
async fn on_set_buy(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::BuySet>()?;
	if let Some(ev) = event_parse {
		let config = shared::config::Config::init();
		let unit_price = shared::utils::string_decimal_to_number(
			&ev.unit_price.to_string(),
			config.chain_decimal as i32,
		);
		let nft = models::trade::Nft {
			amount: ev.amount,
			collection: ev.collection,
			item: ev.item,
		};
		let unit_price_decimal: Decimal128 = unit_price.parse()?;
		let set_buy_params = SetPriceParams {
			block_height: params.block.height,
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			nft,
			trade_id: ev.trade.to_string(),
			unit_price: unit_price_decimal,
			who: hex::encode(ev.who.0),
			end_block: ev.end_block,
			start_block: ev.start_block,
		};
		trade_service::set_buy(set_buy_params, params.db).await?;
	}
	Ok(())
}

//game::PriceSet
async fn on_set_price(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::PriceSet>()?;
	if let Some(ev) = event_parse {
		let nft = models::trade::Nft {
			amount: ev.amount,
			collection: ev.collection,
			item: ev.item,
		};
		let config = shared::config::Config::init();
		let unit_price = shared::utils::string_decimal_to_number(
			&ev.unit_price.to_string(),
			config.chain_decimal as i32,
		);
		let unit_price_decimal: Decimal128 = unit_price.parse()?;

		let set_price_params = SetPriceParams {
			block_height: params.block.height,
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			nft,
			trade_id: ev.trade.to_string(),
			unit_price: unit_price_decimal,
			who: hex::encode(ev.who.0),
			end_block: ev.start_block,
			start_block: ev.end_block,
		};
		trade_service::set_price(set_price_params, params.db).await?;
		services::nft_service::refresh_balance(
			ev.who,
			ev.collection.to_string(),
			ev.item.to_string(),
			params.db,
			params.api,
		)
		.await?;
	}
	Ok(())
}

pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_SET_PRICE, move |params| {
			Box::pin(on_set_price(params))
		}),
		EventHandle::new(EVENT_SET_BUY, move |params| Box::pin(on_set_buy(params))),
		EventHandle::new(EVENT_BOUGHT_ITEM, move |params| {
			Box::pin(on_item_bought(params))
		}),
	]
}
