use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
};
use shared::{
	constant::{
		EVENT_BOUGHT_ITEM, EVENT_SET_BUY, EVENT_SET_PRICE, TRADE_SET_BUY, TRADE_SET_PRICE,
		TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD,
	},
	history_tx, models, BaseDocument, Trade,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi, services,
	workers::{HandleParams, EventHandle},
};

async fn on_item_bought(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemBought>()?;
	if let Some(ev) = event_parse {
		let trade = services::trade_service::get_by_trade_id(params.db, &ev.trade.to_string())
			.await?
			.unwrap();

		// let bundle_of = services::trade::bundle_of(ev.trade, params.api).await?;
		// let trade_item = bundle_of.get(0).unwrap();

		// let is_sold = trade_item.amount == 0;
		let sold = trade.sold.unwrap_or(0) + ev.amount;
		let nft = trade.nft.unwrap();
		let is_sold = sold == nft.amount;

		//update history
		let config = shared::config::Config::init();
		let total_value: u128 = ev.bid_unit_price * u128::from(ev.amount);
		let total_value_decimal: Decimal128 = shared::utils::string_decimal_to_number(
			&total_value.to_string(),
			config.chain_decimal as i32,
		)
		.parse()?;

		let extrinsic_index = params.extrinsic_index.unwrap();
		let history = models::HistoryTx {
			block_height: params.block.height,
			event: EVENT_BOUGHT_ITEM.to_string(),
			event_index: params.ev.index(),
			extrinsic_index,
			from: trade.owner,
			to: Some(hex::encode(ev.who.0)),
			id: None,
			nfts: Some(vec![nft]),
			pool: None,
			tx_hash: None,
			value: Some(total_value_decimal),
			amount: Some(ev.amount),
			price: Some(ev.bid_unit_price.to_string().parse()?),
			trade_id: Some(ev.trade.to_string()),
			source: None,
			trade_type: None,
		};
		services::history_service::upsert(history, params.db).await?;

		//always update trade in last of func
		// update trade
		let trade_db = params.db.collection::<models::Trade>(models::Trade::name().as_str());
		if is_sold {
			trade_db
				.update_one(
					doc! {
						"trade_id": trade.trade_id,
					},
					doc! {
					"$set":{
						// "amount": trade_item.amount,
						"status": TRADE_STATUS_SOLD,
						"sold": sold,
					}
					},
					None,
				)
				.await?;
		} else {
			trade_db
				.update_one(
					doc! {
						"trade_id": trade.trade_id,
					},
					doc! {
							  "$set":{
								  // "amount": trade_item.amount,
					"sold": sold,
							  }
						  },
					None,
				)
				.await?;
		}
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
		let unit_price_decimal: Decimal128 = unit_price.parse()?;
		let trade: Document = Trade {
			id: None,

			nft: Some(models::trade::Nft {
				amount: ev.amount,
				collection: ev.collection,
				item: ev.item,
			}),
			maybe_required: None,
			source: None,
			bundle: None,
			wish_list: None,

			maybe_price: None,
			unit_price: Some(unit_price_decimal),
			price: None,

			owner: hex::encode(ev.who.0),

			start_block: ev.start_block,
			end_block: ev.end_block,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_BUY.to_string(),

			sold: Some(0),
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

		//update history
		let history = history_tx::HistoryTx {
			amount: Some(ev.amount),
			block_height: params.block.height,
			event: TRADE_SET_BUY.to_string(),
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			from: hex::encode(ev.who.0),
			id: None,
			nfts: Some(vec![shared::models::history_tx::Nft {
				amount: ev.amount,
				collection: ev.collection,
				item: ev.item,
			}]),
			pool: None,
			price: Some(unit_price_decimal),
			to: None,
			tx_hash: None,
			value: None,
			trade_id: Some(ev.trade.to_string()),
			source: None,
			trade_type: None,
		};
		services::history_service::upsert(history, params.db).await?;
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
		let trade: Document = Trade {
			id: None,

			nft: Some(nft.clone()),
			maybe_required: None,
			source: None,
			bundle: None,
			wish_list: None,

			owner: hex::encode(ev.who.0),
			start_block: None,
			end_block: None,
			duration: None,

			unit_price: Some(unit_price_decimal),
			maybe_price: None,
			price: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_PRICE.to_string(),

			sold: Some(0),
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

		//update history
		let history = history_tx::HistoryTx {
			amount: Some(ev.amount),
			block_height: params.block.height,
			event: TRADE_SET_PRICE.to_string(),
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			from: hex::encode(ev.who.0),
			id: None,
			nfts: Some(vec![shared::models::history_tx::Nft {
				amount: ev.amount,
				collection: ev.collection,
				item: ev.item,
			}]),
			pool: None,
			price: Some(unit_price_decimal),
			to: None,
			tx_hash: None,
			value: None,
			trade_id: Some(ev.trade.to_string()),
			source: None,
			trade_type: None,
		};
		services::history_service::upsert(history, params.db).await?;

		//refetch balance
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
